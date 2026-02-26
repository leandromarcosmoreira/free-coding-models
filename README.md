<p align="center">
  <img src="https://img.shields.io/npm/v/free-coding-models?color=76b900&label=npm&logo=npm" alt="npm version">
  <img src="https://img.shields.io/node/v/free-coding-models?color=76b900&logo=node.js" alt="node version">
  <img src="https://img.shields.io/npm/l/free-coding-models?color=76b900" alt="license">
  <img src="https://img.shields.io/badge/models-134-76b900?logo=nvidia" alt="models count">
  <img src="https://img.shields.io/badge/providers-17-blue" alt="providers count">
</p>

<h1 align="center">free-coding-models</h1>

<p align="center">
  💬 <a href="https://discord.gg/5MbTnDC3Md">Vamos conversar sobre o projeto no Discord</a>
</p>

<p align="center">

```
1. Crie uma chave de API gratuita (NVIDIA, OpenRouter, Hugging Face, etc.)
2. npm i -g free-coding-models
3. free-coding-models
```

</p>

<p align="center">
  <strong>Encontre os modelos de LLM para codificação mais rápidos em segundos</strong><br>
  <sub>Teste o ping de modelos gratuitos de 17 provedores em tempo real — escolha o melhor para OpenCode, OpenClaw ou qualquer assistente de IA para código</sub>
</p>

<p align="center">
  <img src="demo.gif" alt="free-coding-models demo" width="100%">
</p>

<p align="center">
  <a href="#-recursos">Recursos</a> •
  <a href="#-requisitos">Requisitos</a> •
  <a href="#-instalação">Instalação</a> •
  <a href="#-uso">Uso</a> •
  <a href="#-colunas-tui">Colunas</a> •
  <a href="#-pontuação-de-estabilidade">Estabilidade</a> •
  <a href="#-modelos-de-codificação">Modelos</a> •
  <a href="#-integração-opencode">OpenCode</a> •
  <a href="#-integração-openclaw">OpenClaw</a> •
  <a href="#-como-funciona">Como funciona</a>
</p>

---

## ✨ Recursos

- **🎯 Focado em Codificação** — Apenas modelos LLM otimizados para geração de código, não chat ou visão
- **🌐 Multiprovedor** — 134 modelos de NVIDIA NIM, Groq, Cerebras, SambaNova, OpenRouter, Hugging Face Inference, Replicate, DeepInfra, Fireworks AI, Codestral, Hyperbolic, Scaleway, Google AI, SiliconFlow, Together AI, Cloudflare Workers AI e Perplexity API
- **⚙️ Tela de Configurações** — Pressione `P` para gerenciar chaves de API, habilitar/desabilitar provedores, testar chaves ao vivo e verificar/instalar atualizações manualmente
- **🚀 Pings Paralelos** — Todos os modelos testados simultaneamente via `fetch` nativo
- **📊 Animação em Tempo Real** — Veja a latência aparecer ao vivo no buffer de tela alternativo
- **🏆 Ranking Inteligente** — Os 3 modelos mais rápidos destacados com medalhas 🥇🥈🥉
- **⏱ Monitoramento Contínuo** — Testa o ping de todos os modelos a cada 3 segundos, nunca para
- **📈 Médias Móveis** — Média calculada a partir de TODOS os pings bem-sucedidos desde o início
- **📊 Rastreamento de Uptime** — Porcentagem de pings bem-sucedidos exibida em tempo real
- **📐 Pontuação de Estabilidade** — Pontuação composta de 0 a 100 medindo a consistência (p95, jitter, picos, uptime) — um modelo com média de 400ms estável vence um de 250ms que oscila aleatoriamente para 6s
- **🔄 Auto-tentativa** — Modelos com timeout continuam sendo testados, nada é "desistido"
- **🎮 Seleção Interativa** — Navegue com as setas diretamente na tabela, pressione Enter para agir
- **🔀 Menu de Modo Inicial** — Escolha entre OpenCode e OpenClaw antes do lançamento da TUI
- **💻 Integração OpenCode** — Detecta automaticamente a configuração NIM, define o modelo como padrão e inicia o OpenCode
- **🦞 Integração OpenClaw** — Define o modelo selecionado como provedor padrão em `~/.openclaw/openclaw.json`
- **🎨 Saída Limpa** — Zero poluição de rolagem, a interface permanece aberta até Ctrl+C
- **📶 Indicadores de Status** — UP ✅ · Sem Chave 🔑 · Timeout ⏳ · Sobrecarregado 🔥 · Não Encontrado 🚫
- **🔍 Latência Sem Chave** — Modelos são testados mesmo sem uma chave de API — um status `🔑 NO KEY` confirma que o servidor está acessível com latência real, para que você possa comparar provedores antes de criar uma chave
- **🏷 Filtragem por Camada** — Filtre modelos por letra de camada (S, A, B, C) com a flag `--tier` ou dinamicamente com a tecla `T`
- **⭐ Favoritos Persistentes** — Pressione `F` em uma linha selecionada para fixar/desafixar; favoritos ficam no topo com fundo laranja escuro e uma estrela antes do nome do modelo
- **📊 Analytics com Privacidade (Opcional)** — Eventos PostHog anônimos com consentimento explícito + opção de desativação

---

## 📋 Requisitos

Antes de usar o `free-coding-models`, certifique-se de ter:

1. **Node.js 18+** — Necessário para a API `fetch` nativa
2. **Pelo menos uma chave de API gratuita** — escolha uma ou todas:
   - **NVIDIA NIM** — [build.nvidia.com](https://build.nvidia.com) → Perfil → Chaves de API → Gerar
   - **Groq** — [console.groq.com/keys](https://console.groq.com/keys) → Criar Chave de API
   - **Cerebras** — [cloud.cerebras.ai](https://cloud.cerebras.ai) → Chaves de API → Criar
   - **SambaNova** — [sambanova.ai/developers](https://sambanova.ai/developers) → Portal de desenvolvedores → Chave de API (camada dev generosa)
   - **OpenRouter** — [openrouter.ai/keys](https://openrouter.ai/keys) → Criar chave (50 req/dia, 20/min em `:free`)
   - **Hugging Face Inference** — [huggingface.co/settings/tokens](https://huggingface.co/settings/tokens) → Tokens de Acesso (créditos mensais gratuitos)
   - **Replicate** — [replicate.com/account/api-tokens](https://replicate.com/account/api-tokens) → Criar token (cota dev)
   - **DeepInfra** — [deepinfra.com/login](https://deepinfra.com/login) → Login → Chave de API (camada dev gratuita)
   - **Fireworks AI** — [fireworks.ai](https://fireworks.ai) → Configurações → Tokens de Acesso ($1 em créditos gratuitos)
   - **Mistral Codestral** — [codestral.mistral.ai](https://codestral.mistral.ai) → Chaves de API (30 req/min, 2000/dia — necessário telefone)
   - **Hyperbolic** — [app.hyperbolic.ai/settings](https://app.hyperbolic.ai/settings) → Chaves de API ($1 de teste gratuito)
   - **Scaleway** — [console.scaleway.com/iam/api-keys](https://console.scaleway.com/iam/api-keys) → IAM → Chaves de API (1M de tokens gratuitos)
   - **Google AI Studio** — [aistudio.google.com/apikey](https://aistudio.google.com/apikey) → Obter chave de API (modelos Gemma gratuitos, 14.4K req/dia)
   - **SiliconFlow** — [cloud.siliconflow.cn/account/ak](https://cloud.siliconflow.cn/account/ak) → Chaves de API (cotas variam por modelo gratuito)
   - **Together AI** — [api.together.ai/settings/api-keys](https://api.together.ai/settings/api-keys) → Chaves de API (créditos/promoções variam)
   - **Cloudflare Workers AI** — [dash.cloudflare.com](https://dash.cloudflare.com) → Criar token de API + definir `CLOUDFLARE_ACCOUNT_ID` (Grátis: 10k neurons/dia)
   - **Perplexity API** — [perplexity.ai/settings/api](https://www.perplexity.ai/settings/api) → Chave de API (limites por gastos)
3. **OpenCode** *(opcional)* — [Instale o OpenCode](https://github.com/opencode-ai/opencode) para usar a integração com OpenCode
4. **OpenClaw** *(opcional)* — [Instale o OpenClaw](https://openclaw.ai) para usar a integração com OpenClaw

> 💡 **Dica:** Você não precisa de todos os dezessete provedores. Uma chave é suficiente para começar. Adicione mais tarde pela tela de Configurações (tecla `P`). Modelos sem chave ainda mostram latência real (`🔑 NO KEY`) para que você possa avaliar os provedores antes de se cadastrar.

---

## 📦 Instalação

```bash
# npm (instalação global — recomendado)
npm install -g free-coding-models

# pnpm
pnpm add -g free-coding-models

# bun
bun add -g free-coding-models

# Ou use diretamente com npx/pnpx/bunx
npx free-coding-models SUA_CHAVE_API
pnpx free-coding-models SUA_CHAVE_API
bunx free-coding-models SUA_CHAVE_API
```

---

## 🚀 Uso

```bash
# Apenas execute — mostra um menu inicial para escolher OpenCode ou OpenClaw, solicita chave se não estiver definida
free-coding-models

# Alvo explícito OpenCode CLI (TUI + Enter inicia OpenCode CLI)
free-coding-models --opencode

# Alvo explícito OpenCode Desktop (TUI + Enter define modelo e abre Desktop app)
free-coding-models --opencode-desktop

# Alvo explícito OpenClaw (TUI + Enter define modelo como padrão no OpenClaw)
free-coding-models --openclaw

# Mostrar apenas modelos de alto nível (A+, S, S+)
free-coding-models --best

# Analisar por 10 segundos e mostrar o modelo mais confiável
free-coding-models --fiable

# Desativar analytics anônimo para esta execução
free-coding-models --no-telemetry

# Filtrar modelos por letra de camada
free-coding-models --tier S          # Apenas S+ e S
free-coding-models --tier A          # Apenas A+, A, A-
free-coding-models --tier B          # Apenas B+, B
free-coding-models --tier C          # Apenas C

# Combinar flags livremente
free-coding-models --openclaw --tier S
free-coding-models --opencode --best
```

### Menu de modo inicial

Quando você executa `free-coding-models` sem `--opencode` ou `--openclaw`, você verá um menu interativo:

```
  ⚡ Free Coding Models — Escolha sua ferramenta

  ❯ 💻 OpenCode CLI
       Pressione Enter em um modelo → inicia OpenCode CLI com ele como padrão

     🖥 OpenCode Desktop
       Pressione Enter em um modelo → define modelo e abre OpenCode Desktop app

     🦞 OpenClaw
       Pressione Enter em um modelo → define como padrão na configuração do OpenClaw

  ↑↓ Navegar  •  Enter Selecionar  •  Ctrl+C Sair
```

Use as setas `↑↓` para selecionar, `Enter` para confirmar. A TUI abrirá com o modo escolhido mostrado no cabeçalho.

**Como funciona:**
1. **Fase de ping** — Todos os modelos habilitados são testados em paralelo (até 134 em 17 provedores)
2. **Monitoramento contínuo** — Modelos são testados novamente a cada 3 segundos indefinidamente
3. **Atualizações em tempo real** — Veja as colunas "Latest", "Avg" e "Up%" atualizarem ao vivo
4. **Selecione a qualquer momento** — Use as setas ↑↓ para navegar, pressione Enter em um modelo para agir
5. **Detecção inteligente** — Detecta automaticamente se o NVIDIA NIM está configurado no OpenCode ou OpenClaw

Assistente de configuração (primeira execução — passa por todos os 17 provedores):

```
  🔑 Configuração inicial — Chaves de API
  Insira as chaves para qualquer provedor que deseja usar. Pressione Enter para pular.

  ● NVIDIA NIM
    Chave gratuita em: https://build.nvidia.com
    Perfil → Chaves de API → Gerar
  Insira a chave (ou Enter para pular): nvapi-xxxx

  ● Groq
    Chave gratuita em: https://console.groq.com/keys
    Chaves de API → Criar Chave de API
  Insira a chave (ou Enter para pular): gsk_xxxx

  ● Cerebras
    Chave gratuita em: https://cloud.cerebras.ai
    Chaves de API → Criar
  Insira a chave (ou Enter para pular):

  ● SambaNova
    Chave gratuita em: https://cloud.sambanova.ai/apis
    Chaves de API → Criar ($5 teste gratuito, 3 meses)
  Insira a chave (ou Enter para pular):

  ✅ 2 chave(s) salvas em ~/.free-coding-models.json
  Você pode adicionar ou alterar chaves a qualquer momento com a tecla P na TUI.
```

Você não precisa de todos os dezessete — pule qualquer provedor pressionando Enter. Pelo menos uma chave é necessária.

### Adicionando ou alterando chaves depois

Pressione **`P`** para abrir a tela de Configurações a qualquer momento:

```
  ⚙  Configurações

  Provedores

  ❯ [ ✅ ] NVIDIA NIM              nvapi-••••••••••••3f9a  [Teste ✅]  Camada Gratuita
    [ ✅ ] OpenRouter              (sem chave definida)    [Teste —]   50 req/dia, 20/min
    [ ✅ ] Hugging Face Inference  (sem chave definida)    [Teste —]   Créditos mensais gratuitos

  Instruções de Configuração — NVIDIA NIM
  1) Crie uma conta NVIDIA NIM: https://build.nvidia.com
  2) Perfil → Chaves de API → Gerar
  3) Pressione T para testar sua chave

  ↑↓ Navegar  •  Enter Editar chave / Verificar atualização  •  Espaço Alternar  •  T Testar chave  •  U Atualizar  •  Esc Fechar
```

- **↑↓** — navegar pelos provedores
- **Enter** — entrar no modo de edição de chave (digite a chave, Enter para salvar, Esc para cancelar)
- **Espaço** — habilitar/desabilitar provedor
- **T** — dispara um ping de teste real para verificar se a chave funciona (mostra ✅/❌)
- **U** — verifica manualmente no npm por uma versão mais nova
- **Esc** — fecha configurações e recarrega a lista de modelos

As chaves são salvas em `~/.free-coding-models.json` (permissões `0600`).

A opção de analytics está na mesma tela de Configurações (`P`) como uma linha dedicada (alterne com Enter ou Espaço).
A atualização manual está na mesma tela sob **Maintenance** (Enter para verificar, Enter novamente para instalar).
Os favoritos também são persistidos no mesmo arquivo de configuração.

### Substituição por variáveis de ambiente

Variáveis de ambiente sempre têm prioridade sobre o arquivo de configuração:

```bash
NVIDIA_API_KEY=nvapi-xxx free-coding-models
GROQ_API_KEY=gsk_xxx free-coding-models
CEREBRAS_API_KEY=csk_xxx free-coding-models
OPENROUTER_API_KEY=sk-or-xxx free-coding-models
HUGGINGFACE_API_KEY=hf_xxx free-coding-models
REPLICATE_API_TOKEN=r8_xxx free-coding-models
DEEPINFRA_API_KEY=di_xxx free-coding-models
FIREWORKS_API_KEY=fw_xxx free-coding-models
SILICONFLOW_API_KEY=sk_xxx free-coding-models
TOGETHER_API_KEY=together_xxx free-coding-models
CLOUDFLARE_API_TOKEN=cf_xxx CLOUDFLARE_ACCOUNT_ID=seu_id_conta free-coding-models
PERPLEXITY_API_KEY=pplx_xxx free-coding-models
FREE_CODING_MODELS_TELEMETRY=0 free-coding-models
```

Variáveis de ambiente de telemetria:

- `FREE_CODING_MODELS_TELEMETRY=0|1` — força desativar/ativar analytics
- `FREE_CODING_MODELS_POSTHOG_KEY` — chave de API do projeto PostHog
- `FREE_CODING_MODELS_POSTHOG_HOST` — host opcional (`https://eu.i.posthog.com` por padrão)
- `FREE_CODING_MODELS_TELEMETRY_DEBUG=1` — logs de depuração opcionais

Na primeira execução, o CLI pergunta se aceita analytics anônimo.
Eventos incluem: nome do evento, versão do app, modo selecionado, sistema (`macOS`/`Windows`/`Linux`) e família do terminal.

### Obtenha suas chaves de API gratuitas

**NVIDIA NIM** (44 modelos, camadas S+ → C):
1. Cadastre-se em [build.nvidia.com](https://build.nvidia.com)
2. Vá em Perfil → Chaves de API → Gerar Chave de API
3. Copie — exibida apenas uma vez!

**Groq** (6 modelos, inferência rápida):
1. Cadastre-se em [console.groq.com](https://console.groq.com)
2. Vá em Chaves de API → Criar Chave de API

**Cerebras** (3 modelos, silício ultra-rápido):
1. Cadastre-se em [cloud.cerebras.ai](https://cloud.cerebras.ai)
2. Vá em Chaves de API → Criar

**OpenRouter** (modelos `:free`):
1. Cadastre-se em [openrouter.ai/keys](https://openrouter.ai/keys)
2. Crie chave de API (`sk-or-...`)

**Hugging Face Inference**:
1. Cadastre-se em [huggingface.co/settings/tokens](https://huggingface.co/settings/tokens)
2. Crie Token de Acesso (`hf_...`)

**Replicate**:
1. Cadastre-se em [replicate.com/account/api-tokens](https://replicate.com/account/api-tokens)
2. Crie token de API (`r8_...`)

**DeepInfra**:
1. Cadastre-se em [deepinfra.com/login](https://deepinfra.com/login)
2. Crie chave de API no painel da conta

**Fireworks AI**:
1. Cadastre-se em [fireworks.ai](https://fireworks.ai)
2. Abra Configurações → Tokens de Acesso e crie um token

**Mistral Codestral**:
1. Cadastre-se em [codestral.mistral.ai](https://codestral.mistral.ai)
2. Vá em Chaves de API → Criar

**Hyperbolic**:
1. Cadastre-se em [app.hyperbolic.ai/settings](https://app.hyperbolic.ai/settings)
2. Crie uma chave de API em Configurações

**Scaleway**:
1. Cadastre-se em [console.scaleway.com/iam/api-keys](https://console.scaleway.com/iam/api-keys)
2. Vá em IAM → Chaves de API

**Google AI Studio**:
1. Cadastre-se em [aistudio.google.com/apikey](https://aistudio.google.com/apikey)
2. Crie uma chave de API para endpoints Gemini/Gemma

**SiliconFlow**:
1. Cadastre-se em [cloud.siliconflow.cn/account/ak](https://cloud.siliconflow.cn/account/ak)
2. Crie chave de API em Conta → Chaves de API

**Together AI**:
1. Cadastre-se em [api.together.ai/settings/api-keys](https://api.together.ai/settings/api-keys)
2. Crie uma chave de API em Configurações

**Cloudflare Workers AI**:
1. Cadastre-se em [dash.cloudflare.com](https://dash.cloudflare.com)
2. Crie um token de API e exporte `CLOUDFLARE_API_TOKEN` e `CLOUDFLARE_ACCOUNT_ID`

**Perplexity API**:
1. Cadastre-se em [perplexity.ai/settings/api](https://www.perplexity.ai/settings/api)
2. Crie chave de API (`PERPLEXITY_API_KEY`)

> 💡 **Camadas gratuitas** — cada provedor expõe uma camada dev/gratuita com suas próprias cotas.

---

## 🤖 Modelos de Codificação

**134 modelos de codificação** em 17 provedores e 8 camadas, classificados pelo [SWE-bench Verified](https://www.swebench.com) — o padrão da indústria que mede a resolução de problemas reais do GitHub. As pontuações são informadas pelos provedores, a menos que indicado o contrário.

### NVIDIA NIM (44 modelos)

| Camada | SWE-bench | Modelos |
|------|-----------|--------|
| **S+** ≥70% | GLM 5 (77.8%), Kimi K2.5 (76.8%), Step 3.5 Flash (74.4%), MiniMax M2.1 (74.0%), GLM 4.7 (73.8%), DeepSeek V3.2 (73.1%), Devstral 2 (72.2%), Kimi K2 Thinking (71.3%), Qwen3 Coder 480B (70.6%), Qwen3 235B (70.0%) |
| **S** 60–70% | MiniMax M2 (69.4%), DeepSeek V3.1 Terminus (68.4%), Qwen3 80B Thinking (68.0%), Qwen3.5 400B (68.0%), Kimi K2 Instruct (65.8%), Qwen3 80B Instruct (65.0%), DeepSeek V3.1 (62.0%), Llama 4 Maverick (62.0%), GPT OSS 120B (60.0%) |
| **A+** 50–60% | Mistral Large 675B (58.0%), Nemotron Ultra 253B (56.0%), Colosseum 355B (52.0%), QwQ 32B (50.0%) |
| **A** 40–50% | Nemotron Super 49B (49.0%), Mistral Medium 3 (48.0%), Qwen2.5 Coder 32B (46.0%), Magistral Small (45.0%), Llama 4 Scout (44.0%), Llama 3.1 405B (44.0%), Nemotron Nano 30B (43.0%), R1 Distill 32B (43.9%), GPT OSS 20B (42.0%) |
| **A-** 35–40% | Llama 3.3 70B (39.5%), Seed OSS 36B (38.0%), R1 Distill 14B (37.7%), Stockmark 100B (36.0%) |
| **B+** 30–35% | Ministral 14B (34.0%), Mixtral 8x22B (32.0%), Granite 34B Code (30.0%) |
| **B** 20–30% | R1 Distill 8B (28.2%), R1 Distill 7B (22.6%) |
| **C** <20% | Gemma 2 9B (18.0%), Phi 4 Mini (14.0%), Phi 3.5 Mini (12.0%) |

### Groq (10 modelos)

| Camada | SWE-bench | Modelo |
|------|-----------|-------|
| **S** 60–70% | Kimi K2 Instruct (65.8%), Llama 4 Maverick (62.0%) |
| **A+** 50–60% | QwQ 32B (50.0%) |
| **A** 40–50% | Llama 4 Scout (44.0%), R1 Distill 70B (43.9%) |
| **A-** 35–40% | Llama 3.3 70B (39.5%) |

### Cerebras (7 modelos)

| Camada | SWE-bench | Modelo |
|------|-----------|-------|
| **A+** 50–60% | Qwen3 32B (50.0%) |
| **A** 40–50% | Llama 4 Scout (44.0%) |
| **A-** 35–40% | Llama 3.3 70B (39.5%) |

### Escala de camadas

- **S+/S** — Codificadores de elite (≥60% SWE-bench), melhores para tarefas complexas e refatorações
- **A+/A** — Ótimas alternativas, fortes na maioria das tarefas de codificação
- **A-/B+** — Desempenho sólido, bons para tarefas de programação específicas
- **B/C** — Modelos leves ou antigos, bons para conclusão de código em infraestrutura limitada

### Filtrando por camada

Use `--tier` para focar em uma faixa específica de capacidade:

```bash
free-coding-models --tier S     # Apenas S+ e S (modelos de fronteira)
free-coding-models --tier A     # Apenas A+, A, A- (desempenho sólido)
free-coding-models --tier B     # Apenas B+, B (opções leves)
free-coding-models --tier C     # Apenas C (modelos mínimos)
```

#### Filtragem dinâmica com as teclas E/D

Durante a execução, use as teclas **E** e **D** para ajustar dinamicamente o filtro de camadas:

- **E** (Elevar) — Mostra menos modelos, de camadas superiores (ciclo: Todos → S → A → B → C → Todos)
- **D** (Descer) — Mostra mais modelos, de camadas inferiores (ciclo: Todos → C → B → A → S → Todos)

O filtro de camada atual é mostrado no cabeçalho (ex: `[Tier S]`)

---

## 📊 Colunas TUI

A tabela principal exibe uma linha por modelo com as seguintes colunas:

| Coluna | Atalho | Descrição |
|--------|----------|-------------|
| **Rank** | `R` | Posição baseada na ordenação atual (medalhas para o top 3: 🥇🥈🥉) |
| **Tier** | `Y` | Camada SWE-bench (S+, S, A+, A, A-, B+, B, C) |
| **SWE%** | `S` | Pontuação SWE-bench Verified — padrão da indústria para resolução de issues do GitHub |
| **CTX** | `C` | Tamanho da janela de contexto em milhares de tokens (ex: `128k`) |
| **Model** | `M` | Nome de exibição do modelo (favoritos mostram prefixo ⭐) |
| **Origin** | `N` | Nome do provedor (NIM, Groq, Cerebras, etc.) — pressione `N` para ciclar o filtro |
| **Latest Ping** | `L` | Latência da rodada mais recente em milissegundos |
| **Avg Ping** | `A` | Média móvel de TODOS os pings bem-sucedidos desde o lançamento |
| **Health** | `H` | Status atual: UP ✅, SEM CHAVE 🔑, Timeout ⏳, Sobrecarregado 🔥, Não Encontrado 🚫 |
| **Verdict** | `V` | Veredito de saúde baseado em latência média + estabilidade (veja abaixo) |
| **Stability** | `B` | Pontuação composta de consistência 0–100 (veja [Pontuação de Estabilidade](#-pontuação-de-estabilidade)) |
| **Up%** | `U` | Uptime — porcentagem de pings bem-sucedidos em relação ao total |

### Valores de veredito

A coluna Veredito combina latência média com análise de estabilidade:

| Veredito | Significado |
|---------|---------|
| **Perfect** | Média < 400ms com p95/jitter estáveis |
| **Normal** | Média < 1000ms, respostas consistentes |
| **Slow** | Média 1000–2000ms |
| **Spiky** | Boa média, mas latência de cauda errática (p95 >> média) |
| **Very Slow** | Média 2000–5000ms |
| **Overloaded** | Servidor retornou 429/503 (limite de taxa ou capacidade) |
| **Unstable** | Estava ok, mas agora dá timeout, ou média > 5000ms |
| **Not Active** | Nenhum ping bem-sucedido ainda |
| **Pending** | Primeiro ping ainda em curso |

---

## 📐 Pontuação de Estabilidade

A coluna **Stability** (ordene com a tecla `B`) mostra uma pontuação composta de 0 a 100 que responde: *"Quão consistente e previsível é este modelo?"*

A latência média sozinha pode enganar — um modelo com média de 250ms que oscila para 6 segundos *parece* mais lento na prática do que um modelo estável de 400ms. A pontuação de estabilidade captura isso.

### Fórmula

Quatro sinais são normalizados de 0 a 100 cada, depois combinados com pesos:

```
Estabilidade = 0.30 × p95_score
             + 0.30 × jitter_score
             + 0.20 × spike_score
             + 0.20 × reliability_score
```

| Componente | Peso | O que mede | Como é normalizado |
|-----------|--------|-----------------|---------------------|
| **Latência p95** | 30% | Latência de cauda — os piores 5% dos tempos | `100 × (1 - p95 / 5000)`, limitado a 0–100 |
| **Jitter (σ)** | 30% | Respostas erráticas — desvio padrão dos pings | `100 × (1 - jitter / 2000)`, limitado a 0–100 |
| **Taxa de pico** | 20% | Fração de pings acima de 3000ms | `100 × (1 - picos / total_pings)` |
| **Confiabilidade** | 20% | Uptime — fração de pings HTTP 200 | Porcentagem direta de uptime (0–100) |

### Código de cores

| Pontuação | Cor | Interpretação |
|-------|-------|----------------|
| **80–100** | Verde | Sólido — muito consistente, confiável |
| **60–79** | Ciano | Bom — variação ocasional, mas geralmente estável |
| **40–59** | Amarelo | Instável — inconsistência perceptível |
| **< 40** | Vermelho | Não confiável — picos ou falhas frequentes |
| **—** | Dim | Sem dados ainda (nenhum ping bem-sucedido) |

### Exemplo

Dois modelos com latência média similar, experiências reais muito diferentes:

```
Modelo A:  média 250ms,  p95 6000ms,  jitter 1800ms  →  Estabilidade ~30  (vermelho)
Modelo B:  média 400ms,  p95  650ms,  jitter  120ms  →  Estabilidade ~85  (verde)
```

O Modelo B é a melhor escolha apesar da média maior — ele não travará seu fluxo de trabalho aleatoriamente.

> 💡 **Dica:** Ordene por Estabilidade (tecla `B`) após alguns minutos para encontrar os modelos com desempenho mais previsível.

---

## 🔌 Integração OpenCode

**A maneira mais fácil** — deixe o `free-coding-models` fazer tudo:

1. **Execute**: `free-coding-models --opencode` (ou escolha OpenCode no menu inicial)
2. **Aguarde** os pings (status verde ✅)
3. **Navegue** com as setas ↑↓ até seu modelo preferido
4. **Pressione Enter** — a ferramenta automaticamente:
   - Detecta se o NVIDIA NIM está configurado no OpenCode
   - Define o modelo selecionado como padrão em `~/.config/opencode/opencode.json`
   - Inicia o OpenCode com o modelo pronto para uso

### Panes de sub-agente tmux

Quando iniciado de uma sessão `tmux`, o `free-coding-models` agora adiciona automaticamente um argumento `--port` para que o OpenCode possa criar sub-agentes em panes.

- Prioridade 1: reutilizar `OPENCODE_PORT` se for válida e estiver livre
- Prioridade 2: escolher automaticamente a primeira porta livre em `4096-5095`

Você pode forçar uma porta específica:

```bash
OPENCODE_PORT=4098 free-coding-models --opencode
```

### Configuração Manual do OpenCode (Opcional)

Crie ou edite `~/.config/opencode/opencode.json`:

```json
{
  "provider": {
    "nvidia": {
      "npm": "@ai-sdk/openai-compatible",
      "name": "NVIDIA NIM",
      "options": {
        "baseURL": "https://integrate.api.nvidia.com/v1",
        "apiKey": "{env:NVIDIA_API_KEY}"
      }
    }
  },
  "model": "nvidia/deepseek-ai/deepseek-v3.2"
}
```

Então defina a variável de ambiente:

```bash
export NVIDIA_API_KEY=nvapi-xxxx-sua-chave-aqui
```

Execute `/models` no OpenCode e selecione o provedor **NVIDIA NIM** e o modelo escolhido.

### Fallback de Instalação Automática

Se o NVIDIA NIM ainda não estiver configurado no OpenCode, a ferramenta:
- Mostra instruções de instalação no terminal
- Cria um arquivo `prompt` em `$HOME/prompt` com a configuração exata
- Inicia o OpenCode, que detectará e exibirá o prompt automaticamente

---

## 🦞 Integração OpenClaw

OpenClaw é um daemon de agente de IA autônomo. O `free-coding-models` pode configurá-lo para usar modelos NVIDIA NIM como seu provedor padrão — sem download ou configuração local, tudo roda via API remota do NIM.

### Início Rápido

```bash
free-coding-models --openclaw
```

Ou execute sem flags e escolha **OpenClaw** no menu inicial.

1. **Aguarde** os pings dos modelos
2. **Navegue** com as setas ↑↓ até seu modelo preferido
3. **Pressione Enter** — a ferramenta automaticamente:
   - Lê `~/.openclaw/openclaw.json`
   - Adiciona o bloco do provedor `nvidia` se estiver faltando
   - Define `agents.defaults.model.primary` para `nvidia/<model-id>`
   - Salva a configuração e mostra os próximos passos

### O que é escrito na config do OpenClaw

```json
{
  "models": {
    "providers": {
      "nvidia": {
        "baseUrl": "https://integrate.api.nvidia.com/v1",
        "api": "openai-completions"
      }
    }
  },
  "env": {
    "NVIDIA_API_KEY": "nvapi-xxxx-sua-chave"
  },
  "agents": {
    "defaults": {
      "model": {
        "primary": "nvidia/deepseek-ai/deepseek-v3.2"
      },
      "models": {
        "nvidia/deepseek-ai/deepseek-v3.2": {}
      }
    }
  }
}
```

> ⚠️ **Nota:** `providers` deve estar dentro de `models.providers`. Um bloco `providers` na raiz é ignorado pelo OpenClaw.

> ⚠️ **Nota:** O modelo também deve estar listado em `agents.defaults.models` (allowlist). Sem isso, o OpenClaw rejeita o modelo.

### Aplicando a configuração do OpenClaw

O gateway do OpenClaw **recarrega automaticamente** as mudanças. Para aplicar manualmente:

```bash
# Aplicar via CLI
openclaw models set nvidia/deepseek-ai/deepseek-v3.2

# Ou execute o assistente interativo
openclaw configure
```

> 💡 **Por que usar modelos NIM remotos com OpenClaw?** O NVIDIA NIM serve modelos via uma API rápida — sem necessidade de GPU local ou limites de VRAM. Você obtém modelos de ponta sem baixar nada.

### Patcheando o OpenClaw para suporte total de modelos NVIDIA

**Problema:** Por padrão, o OpenClaw permite apenas alguns modelos NVIDIA específicos.

**Solução:** Patcheie a configuração do OpenClaw para adicionar TODOS os 47 modelos NVIDIA do `free-coding-models` à allowlist:

```bash
# Do diretório do pacote free-coding-models
node patch-openclaw.js
```

Este script:
- Faz backup do `models.json` e `openclaw.json`
- Adiciona todos os 47 modelos NVIDIA com limites corretos
- Preserva modelos e configurações existentes

---

## ⚙️ Como funciona

```
┌──────────────────────────────────────────────────────────────────┐
│  1. Entra no buffer de tela alternativo (como vim/htop)          │
│  2. Testa o PING de TODOS os modelos em paralelo                 │
│  3. Exibe tabela em tempo real com Médias/Estabilidade/Uptime     │
│  4. Re-testa TODOS os modelos a cada 3 segundos (sempre)         │
│  5. Atualiza médias móveis + pontuações de estabilidade          │
│  6. Usuário navega com ↑↓ e seleciona com Enter                  │
│  7. No Enter (OpenCode): define modelo, inicia OpenCode          │
│  8. No Enter (OpenClaw): atualiza ~/.openclaw/openclaw.json      │
└──────────────────────────────────────────────────────────────────┘
```

---

## 📋 Referência da API

**Variáveis de ambiente (sobrepõem o arquivo de config):**

| Variável | Descrição |
|----------|-------------|
| `NVIDIA_API_KEY` | Chave NVIDIA NIM |
| `GROQ_API_KEY` | Chave Groq |
| `CEREBRAS_API_KEY` | Chave Cerebras |
| `SAMBANOVA_API_KEY` | Chave SambaNova |
| `OPENROUTER_API_KEY` | Chave OpenRouter |
| `HUGGINGFACE_API_KEY` / `HF_TOKEN` | Token Hugging Face |
| `REPLICATE_API_TOKEN` | Token Replicate |
| `DEEPINFRA_API_KEY` / `DEEPINFRA_TOKEN` | Chave DeepInfra |
| `CODESTRAL_API_KEY` | Chave Mistral Codestral |
| `HYPERBOLIC_API_KEY` | Chave Hyperbolic |
| `SCALEWAY_API_KEY` | Chave Scaleway |
| `GOOGLE_API_KEY` | Chave Google AI Studio |
| `SILICONFLOW_API_KEY` | Chave SiliconFlow |
| `TOGETHER_API_KEY` | Chave Together AI |
| `CLOUDFLARE_API_TOKEN` / `CLOUDFLARE_API_KEY` | Token/chave Cloudflare |
| `CLOUDFLARE_ACCOUNT_ID` | ID da conta Cloudflare |
| `PERPLEXITY_API_KEY` / `PPLX_API_KEY` | Chave Perplexity |
| `FREE_CODING_MODELS_TELEMETRY` | `0` desativa, `1` ativa analytics |
| `FREE_CODING_MODELS_POSTHOG_KEY` | Chave PostHog |

**Arquivo de config:** `~/.free-coding-models.json` (criado automaticamente, permissões `0600`)

**Atalhos de teclado (TUI principal):**
- **↑↓** — Navegar modelos
- **Enter** — Selecionar modelo
- **R/Y/O/M/L/A/S/N/H/V/B/U** — Ordenar por Rank/Camada/Origem/Modelo/PingRecente/Média/SWE/Contexto/Saúde/Veredito/Estabilidade/Uptime
- **F** — Alternar favorito (⭐ na coluna Model, fixado no topo)
- **T** — Ciclar filtro de camada (Tudo → S+ → S → A+ → A → A- → B+ → B → C → Tudo)
- **Z** — Ciclar modo (OpenCode CLI → OpenCode Desktop → OpenClaw)
- **P** — Abrir Configurações (gerenciar chaves, provedores, analytics, atualizações)
- **W** — Diminuir intervalo de ping (pings mais rápidos)
- **X** — Aumentar intervalo de ping (pings mais lentos)
- **K** / **Esc** — Mostrar/ocultar ajuda
- **Ctrl+C** — Sair

---

## 🔧 Desenvolvimento

```bash
git clone https://github.com/vava-nessa/free-coding-models
cd free-coding-models
npm install
npm start -- SUA_CHAVE_API
```

---

## 📄 Licença

MIT © [vava](https://github.com/vava-nessa)

---

<p align="center">
  <sub>Construído com ☕ e 🌹 por <a href="https://github.com/vava-nessa">vava</a></sub>
</p>

## 📬 Contribua
Contribuições são bem-vindas! Sinta-se à vontade para abrir issues, enviar pull requests ou se envolver no projeto.

## 📧 Suporte

Para perguntas ou problemas, abra uma [issue no GitHub](https://github.com/vava-nessa/free-coding-models/issues).

💬 Vamos conversar sobre o projeto no Discord: https://discord.gg/5MbTnDC3Md
