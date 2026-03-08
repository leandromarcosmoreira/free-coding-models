#!/usr/bin/env node
/**
 * @file patch_docker.js
 * @description Patch OpenClaw (Docker version) to allow all NVIDIA models
 */

import { readFileSync, writeFileSync, existsSync } from 'fs'
import { join } from 'path'
import { nvidiaNim } from './sources.js'

// Paths relative to project root
const MODELS_JSON = join(process.cwd(), '..', 'data', 'openclaw', 'agents', 'main', 'agent', 'models.json')
const OPENCLAW_JSON = join(process.cwd(), '..', 'config', 'openclaw.json')

console.log('🦞 Patching OpenClaw (Docker) for full NVIDIA model support...\n')

// ─── Helper functions ───────────────────────────────────────────────────────────
function getModelConfig(tier) {
  if (tier === 'S+' || tier === 'S') return { contextWindow: 128000, maxTokens: 8192 }
  if (tier === 'A+') return { contextWindow: 131072, maxTokens: 4096 }
  if (tier === 'A' || tier === 'A-') return { contextWindow: 131072, maxTokens: 4096 }
  return { contextWindow: 32768, maxTokens: 2048 }
}

// ─── Patch models.json ──────────────────────────────────────────────────────────
console.log('📄 Patching models.json...')
if (existsSync(MODELS_JSON)) {
  let modelsConfig;
  try {
    modelsConfig = JSON.parse(readFileSync(MODELS_JSON, 'utf8'))
    
    if (!modelsConfig.providers) modelsConfig.providers = {}
    if (!modelsConfig.providers.nvidia) {
      modelsConfig.providers.nvidia = {
        baseUrl: 'https://integrate.api.nvidia.com/v1',
        api: 'openai-completions',
        models: []
      }
    }

    const existingModelIds = new Set(modelsConfig.providers.nvidia.models.map(m => m.id))
    let addedCount = 0
    for (const [modelId, label, tier] of nvidiaNim) {
      if (!existingModelIds.has(modelId)) {
        const config = getModelConfig(tier)
        const isThinking = modelId.includes('thinking')
        modelsConfig.providers.nvidia.models.push({
          id: modelId,
          name: label,
          contextWindow: config.contextWindow,
          maxTokens: config.maxTokens,
          reasoning: isThinking,
          input: ['text'],
          cost: { input: 0, output: 0, cacheRead: 0, cacheWrite: 0 }
        })
        addedCount++
      }
    }
    writeFileSync(MODELS_JSON, JSON.stringify(modelsConfig, null, 2))
    console.log(`  ✅ Added ${addedCount} models to ${MODELS_JSON}`)
  } catch (err) {
    console.error('  ✖ Error patching models.json:', err.message)
  }
} else {
  console.log(`  ℹ models.json not found at ${MODELS_JSON}, skipping...`)
}

// ─── Patch openclaw.json ────────────────────────────────────────────────────────
console.log('\n📄 Patching openclaw.json...')
if (existsSync(OPENCLAW_JSON)) {
  let openclawConfig;
  try {
    openclawConfig = JSON.parse(readFileSync(OPENCLAW_JSON, 'utf8'))
    
    if (!openclawConfig.models) openclawConfig.models = {}
    if (!openclawConfig.models.providers) openclawConfig.models.providers = {}
    if (!openclawConfig.models.providers.nvidia) {
      openclawConfig.models.providers.nvidia = {
        baseUrl: 'https://integrate.api.nvidia.com/v1',
        api: 'openai-completions',
        models: []
      }
    }

    const existingOpenClawModelIds = new Set((openclawConfig.models.providers.nvidia.models || []).map(m => m.id))
    let addedOpenClawCount = 0
    for (const [modelId, label, tier] of nvidiaNim) {
      if (!existingOpenClawModelIds.has(modelId)) {
        const config = getModelConfig(tier)
        openclawConfig.models.providers.nvidia.models.push({
          id: modelId,
          name: label,
          contextWindow: config.contextWindow,
          maxTokens: config.maxTokens
        })
        addedOpenClawCount++
      }
    }
    writeFileSync(OPENCLAW_JSON, JSON.stringify(openclawConfig, null, 2))
    console.log(`  ✅ Added ${addedOpenClawCount} models to ${OPENCLAW_JSON}`)
  } catch (err) {
    console.error('  ✖ Error patching openclaw.json:', err.message)
  }
}

console.log('\n✨ Patch complete!')
