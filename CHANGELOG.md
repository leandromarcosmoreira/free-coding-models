# Registro de Alterações (Changelog)

---

## 0.1.67

### Adicionado

- **Pontuação de Estabilidade** — nova métrica composta de 0–100 combinando latência p95 (30%), jitter/σ (30%), taxa de picos (20%) e uptime (20%). Exibida como uma coluna codificada por cores na TUI (verde ≥80, ciano ≥60, amarelo ≥40, vermelho <40).
- **Latência p95** (`getP95`) — latência do 95º percentil de pings bem-sucedidos. Responde "95% das solicitações são mais rápidas que X ms."
- **Jitter** (`getJitter`) — desvio padrão da latência. Baixo jitter = previsível, alto jitter = errático/instável.
- **Veredito "Spiky" (Instável)** — novo veredito que captura modelos com boa latência média, mas péssima latência de cauda (picos p95). Um modelo com média de 250ms, mas p95 de 6000ms agora é sinalizado como "Spiky 📈" em vez de "Perfect 🚀".
- **Ordenação por Estabilidade** — pressione `B` para ordenar pela pontuação de estabilidade. Os modelos mais estáveis sobem para o topo. A tecla `B` agora está listada nas chaves de ordenação da barra de rodapé.
- 24 novos testes unitários cobrindo p95, jitter, pontuação de estabilidade, veredito Spiky e ordenação por estabilidade.
- **README: Tabela de referência de colunas TUI** — tabela completa de 12 colunas documentando cada coluna (Posição, Camada, SWE%, Modelo, Origem, Último, Média, Saúde, Veredito, Estabilidade, Contexto, Up%).
- **README: Seção de Pontuação de Estabilidade** — documenta a fórmula, pesos, limites de cores e um exemplo de cálculo.
- **README: Tabela de valores de Veredito** — lista todas as 7 categorias de veredito com seus emojis, significados e critérios.

### Alterado

- **Coluna "Stab" renomeada para "Stability"** — cabeçalho da coluna ampliado de 6 para 11 caracteres; o texto do cabeçalho agora lê `StaBility` com a letra da tecla de ordenação `B` em maiúsculo, negrito e amarelo.
- **Coluna SWE%: Gradiente de cores de 8 bandas** — substituído o antigo esquema de cores de 3 bandas (verde ≥50, amarelo ≥30, fraco caso contrário) por um gradiente de 8 bandas correspondente à `TIER_COLOR`: ≥70% verde neon brilhante, ≥60% verde, ≥50% verde-amarelo, ≥40% amarelo, ≥35% âmbar, ≥30% laranja-vermelho, ≥20% vermelho, <20% vermelho escuro.
- `getVerdict()` agora leva em conta a estabilidade: modelos na faixa de média "Perfect" ou "Normal" são rebaixados para "Spiky" quando o p95 mostra latência de cauda extrema (requer ≥3 pings para evitar falsos positivos).
- `findBestModel()` agora usa uma ordenação de 4 chaves: status → latência média → pontuação de estabilidade → uptime (era de 3 chaves: status → média → uptime).
- `sortResults()` suporta a nova coluna `'stability'`.
- `VERDICT_ORDER` atualizado para incluir "Spiky" entre "Slow" e "Very Slow".
- **README: atalhos de teclado** atualizados para incluir `B` para ordenação por Estabilidade; diagrama "Como funciona" atualizado.
- **Intervalo de ping padrão → 3 segundos** (era 2s) para um ritmo padrão mais calmo; ainda ajustável com as teclas W/X.
- **Cores de veredito unificadas com o gradiente TIER_COLOR** — Perfect (ciano-verde) → Normal (lima) → Spiky (verde-amarelo) → Slow (laranja) → Very Slow (laranja-vermelho) → Overloaded (vermelho) → Unstable (vermelho escuro) → Unusable (vermelho mais escuro). Ordenação do melhor para o pior no código.
- **Limpeza do rodapé** — Removida a linha de aviso BETA TUI. Renomeado "Join our Discord" para apenas "Discord" e colocado ao lado de Contributors na linha "Made with love".
- **Cores dos links do rodapé** — Estrela no GitHub: amarelo, Contribuidores: laranja, Discord: roxo claro. O atalho Ctrl+C para Sair foi movido para o final da linha "Made with love".
- **URL simples do Discord** — Mostra `Discord → https://discord.gg/5MbTnDC3Md` para que terminais sem suporte a links OSC 8 ainda possam ver a URL.
- **Estilização da Ajuda K** — Alterado de um selo com fundo verde para texto verde neon (`rgb(0,255,80)`) sem fundo.
- **Estilização do Modo Z** — Cor laranja-avermelhada (`rgb(255,100,50)`) correspondente à marca OpenClaw.
- **Estilização da linha de seleção** — Fundos mais escuros: linhas favoritas `bgRgb(35,20,0)`, linhas do cursor `bgRgb(50,0,60)`. Nome do modelo e Origem renderizados em branco negrito quando selecionados.
- **README** — Atualizadas todas as referências de intervalo de ping de 2s para 3s; removida a linha de aviso BETA.

### Corrigido

- **Alinhamento de colunas: largura dos emojis de Saúde/Status** — A coluna de Saúde usava `.padEnd()` que contava erroneamente a largura dos emojis (✅, 🔥, ⏳ etc. têm 2 colunas de terminal, mas eram contados como menos). Mudado para `padEndDisplay()` para que as colunas Veredito, Estabilidade e Up% agora se alinhem corretamente.
- **Emojis de veredito movidos para o final do texto** — os emojis agora aparecem após a palavra (ex: `Perfect 🚀` em vez de `🚀 Perfect`) para um alinhamento à esquerda mais limpo.
- **Marcadores de células vazias** — alterados de um único `—` para `———` nas colunas Último Ping, Média de Ping e Estabilidade para que as células vazias tenham mais peso visual e não pareçam espaço em branco.

---

## 0.1.66

### Adicionado

- Adicionados 4 novos provedores: SiliconFlow, Together AI, Cloudflare Workers AI e Perplexity API.
- Adicionados 23 modelos dos provedores nessas novas integrações (endpoints compatíveis com OpenAI + metadados de onboarding nas configurações).
- Adicionada orientação de configuração específica para a Cloudflare nas Configurações, incluindo o requisito explícito de `CLOUDFLARE_ACCOUNT_ID`.

### Alterado

- Suporte estendido para provedores/ambientes em config e tempo de execução (`SILICONFLOW_API_KEY`, `TOGETHER_API_KEY`, `CLOUDFLARE_API_TOKEN`/`CLOUDFLARE_API_KEY`, `PERPLEXITY_API_KEY`/`PPLX_API_KEY`).
- Auto-configuração estendida do provedor OpenCode Desktop para SiliconFlow, Together AI, Cloudflare Workers AI e Perplexity API.
- README atualizado para refletir os totais atuais de provedores/modelos (17 provedores / 134 modelos) e documentação expandida de configuração de chaves + variáveis de ambiente.
- Overlays `P` (Configurações) e `K` (Ajuda) atualizados com painéis de fundo escuro dedicados (distintos da tabela principal) para uma separação visual mais clara.

### Corrigido

- Corrigida a rolagem da lista de modelos e regressão da UX de alternância de favoritos introduzida após a versão `0.1.65` (estabilidade de cursor/rolagem ao desfixar favoritos, últimas linhas alcançáveis).
- Corrigida a usabilidade dos overlays em terminais pequenos: `K` (Ajuda) e `P` (Configurações) agora usam rolagem de viewport para que todo o conteúdo e as linhas superiores permaneçam acessíveis.
- Corrigida a navegação pelo teclado da tabela principal para dar a volta: pressionar Para Cima na primeira linha pula para a última linha, e pressionar Para Baixo na última linha pula para a primeira linha.

---

## 0.1.65

### Adicionado

- Adicionados favoritos de modelos persistentes com alternância pela tecla `F`, marcador de estrela na coluna Modelo, destaque de favorito em laranja escuro e comportamento fixado no topo.
- Adicionado fluxo de manutenção de atualização manual nas Configurações (`P`): verificar atualizações do npm sob demanda e instalar diretamente da tela de configurações.
- Overlay de ajuda `K` expandido com atalhos de teclado completos (TUI principal + configurações) e exemplos de uso de flags CLI.

### Alterado

- Favoritos agora permanecem visíveis e fixados independentemente do tipo de ordenação ativo ou de filtros de camada/origem.
- Esquema de configuração estendido (`~/.free-coding-models.json`) com um array `favorites` persistido (entradas `providerKey/modelId`).
- Documentação do README atualizada para favoritos, atualizações manuais, atalhos de configurações e estrutura de configuração.

---

## 0.1.64

### Adicionado

- Adicionados 4 novos provedores gratuitos: Hugging Face Inference, Replicate, DeepInfra e Fireworks AI (modelos, manuseio de chaves, verificações de saúde, integração com Configurações).
- Adicionadas linhas de provedores nas Configurações (`P`) mais ricas com resumo de limite de taxa em linha e status de teste de chave de API ao vivo.

### Alterado

- O lançamento do OpenCode agora detecta `tmux` e auto-injeta `--port` (`OPENCODE_PORT` se estiver livre, caso contrário a primeira disponível em `4096-5095`) para que os painéis de sub-agentes funcionem de forma confiável.
- Conjunto de modelos gratuitos do OpenRouter atualizado para incluir `qwen/qwen3-coder:480b-free`, `mistralai/devstral-2-free` e `mimo-v2-flash-free`.
- Adicionada entrada ajustada para codificação SambaNova `Llama3-Groq`.
- Documentação de setup/config e suporte a variáveis de ambiente atualizados para novos provedores (`HUGGINGFACE_API_KEY`/`HF_TOKEN`, `REPLICATE_API_TOKEN`, `DEEP_INFRA_API_KEY`/`DEEPINFRA_TOKEN`).
- Os pings do Replicate agora usam o formato de solicitação `/v1/predictions`; o lançamento do OpenCode para o Replicate é protegido com uma mensagem clara de apenas monitoramento.
- O painel inferior das Configurações agora mostra as etapas de onboarding do provedor (URL de cadastro + fluxo de criação/teste de chave) em vez de detalhes da lista de modelos.
- Documentado em `AGENTS.md` que as entradas superiores do changelog devem permanecer limpas para reutilização direta nas notas de lançamento do GitHub.

### Corrigido

- O estado desativado das Configurações/onboarding agora usa um cruzamento vermelho explícito (`❌`) em vez de um glifo de quadrado cinza para melhor compatibilidade com fontes de terminal.

---

## 0.1.63

### Alterado

- Telemetria de webhook substituída pela API de captura do PostHog (`/i/v0/e/`) e mantido o consentimento explícito + opção de exclusão `--no-telemetry`.
- Adicionada identidade de telemetria anônima persistente na configuração (`telemetry.anonymousId`) para contagens de uso anônimas estáveis.
- Adicionada UX de tela de consentimento de telemetria: onboarding ASCII personalizado, mensagens de privacidade explícitas e ação padrão de "Aceitar e Continuar".
- Adicionado alternador de telemetria nas Configurações (`P`) e controles de variáveis de ambiente documentados: `FREE_CODING_MODELS_TELEMETRY`, `FREE_CODING_MODELS_POSTHOG_KEY`, `FREE_CODING_MODELS_POSTHOG_HOST`.
- Adicionados campos de metadados de telemetria: `app_version`, `system` (`macOS`/`Windows`/`Linux`) e `terminal` (Terminal.app/iTerm2/kitty/etc. com fallback).
- Adicionado modo de depuração de telemetria com `FREE_CODING_MODELS_TELEMETRY_DEBUG=1` (rastreios stderr para estados enviado/pulado/erro).
- Comportamento de segurança de telemetria endurecido: falhas de analytics permanecem não bloqueantes e execuções sem TTY não sobrescrevem mais o consentimento armazenado.
- Corrigido o renderizador de consentimento para evitar efeitos colaterais de limpeza de tela cheia e preservar a visibilidade do cabeçalho em vários terminais.
- Link de contribuidores do rodapé da TUI atualizado para apontar para o gráfico de contribuidores do repositório.

---

## 0.1.61

### Alterado — Rodapé TUI & UX

- **A linha "Made with" agora é rosa**: toda a frase "Made with 💖 & ☕ by vava-nessa" agora é renderizada em rosa suave (`chalk.rgb(255,150,200)`), incluindo o link clicável do nome do autor, tornando-a visualmente distinta do resto do rodapé.
- **O selo `K Help` agora é ultra visível**: alterado de fundo verde simples para verde brilhante (`bgGreenBright`) com **texto em negrito preto** — alto contraste, destaca-se imediatamente em um relance na linha de dicas do rodapé.
- **A tecla `P` fecha as Configurações**: pressionar `P` novamente enquanto estiver na tela de Configurações agora a fecha (mesmo comportamento que `Esc`). Anteriormente, apenas `Esc` funcionava. Ambas as teclas agora acionam a mesma lógica de fechamento + reconstrução de provedor.

---

## 0.1.60

### Alterado — Rodapé TUI

- **URL do Discord agora mostrada em texto simples**: após o hiperlink clicável "Join our Discord", a URL bruta `https://discord.gg/5MbTnDC3Md` agora é impressa em ciano, separada por `→`. Isso ajuda os usuários em terminais que não suportam links clicáveis OSC 8 a ainda verem e copiarem/colarem a URL.

---

## 0.1.59

### Alterado — Rodapé TUI

- **Selo `K Help` no rodapé agora é verde brilhante**: anteriormente era apenas texto simples, agora é renderizado como `chalk.bgGreen.black.bold(' K Help ')` para que seja imediatamente visível na linha de dicas do rodapé.

---

## 0.1.58

### Alterado — TUI

- **Emoji de timeout atualizado**: substituído `⏱` por `⏳` em todos os lugares na TUI (exibição de timeout de ping).

---

## 0.1.57

### Alterado — Rodapé TUI

- **Texto do link do Discord encurtado**: "Join our Discord" substitui o rótulo anterior mais longo — rodapé mais limpo, o mesmo hiperlink OSC 8 clicável.

---

## 0.1.56

### Alterado — Rodapé TUI

- **Rodapé limpo e reestruturado**: removidas as linhas duplicadas/bagunçadas deixadas pelo agente da versão 0.1.54; consolidado em duas linhas de rodapé limpas:
  - Linha 1: `Made with 💖 & ☕ by vava-nessa  •  ⭐ Star on GitHub` (links clicáveis)
  - Linha 2: `💬 Join our Discord  •  ⚠ BETA TUI — might crash or have problems`
- **Aviso BETA adicionado ao rodapé da TUI**: selo `⚠ BETA TUI` em amarelo com um aviso de texto simples, sempre visível na parte inferior do app TUI.
- **Convite do Discord no rodapé da TUI**: hiperlink clicável OSC 8 adicionado diretamente no rodapé (antes estava apenas no README).

---

## 0.1.55

### Alterado — README & Documentação

- **README atualizado para 9 provedores / 101 modelos**: selos, lista de provedores, seção de Suporte e seção de Requisitos, todos atualizados para refletir o novo estado após a versão 0.1.54.
- **Bloco de cabeçalho do Discord reformatado**: substituído o banner de entrada por um link simples `💬 Let's talk about the project on Discord`.
- **Aviso BETA adicionado ao README**: `⚠️ free-coding-models is a BETA TUI — expect rough edges and occasional crashes` em linha adicionado à linha do link da documentação na seção de Suporte.

---

## 0.1.54

### Adicionado — Provedores & Modelos

**5 novos provedores** (9 no total, 101 modelos):

- **OpenRouter** — 8 modelos de codificação gratuitos via a camada de cota `:free` (20 req/min, 50 req/dia compartilhado). Inclui Qwen3 Coder, Step 3.5 Flash, DeepSeek R1 0528, GPT OSS 120B/20B, Nemotron Nano 30B, Llama 3.3 70B. Prefixo da chave: `sk-or-`
- **Mistral Codestral** — endpoint de codificação dedicado (`codestral.mistral.ai`), modelo `codestral-latest`, 30 req/min / 2.000 req/dia. Chave de API separada da plataforma principal da Mistral. Prefixo da chave: `csk-`
- **Hyperbolic** — $1 em créditos de teste gratuito. 10 modelos: Qwen3 Coder 480B, DeepSeek R1 0528, Kimi K2, GPT OSS 120B, Qwen3 235B, Qwen3 80B Instruct, DeepSeek V3 0324, Qwen2.5 Coder 32B, Llama 3.3 70B, Llama 3.1 405B. Prefixo da chave: `eyJ`
- **Scaleway** — 1 milhão de tokens gratuitos. 7 modelos: Devstral 2 123B, Qwen3 235B, GPT OSS 120B, Qwen3 Coder 30B, Llama 3.3 70B, R1 Distill 70B, Mistral Small 3.2. Prefixo da chave: `scw-`
- **Google AI Studio** — modelos Gemma 3 gratuitos (14.400 req/day, 30 req/min). Gemma 3 27B / 12B / 4B via o endpoint compatível com OpenAI `generativelanguage.googleapis.com/v1beta/openai`. Prefixo da chave: `AIza`

**Novos modelos em provedores existentes:**

- **Groq**: GPT OSS 120B (`openai/gpt-oss-120b`), GPT OSS 20B (`openai/gpt-oss-20b`), Qwen3 32B (`qwen/qwen3-32b`)
- **Cerebras**: GLM 4.6 (`glm-4.6`) de Z.ai — 10 req/min, 100 req/day
- **SambaNova**: DeepSeek V3.1 Terminus (`deepseek-ai/DeepSeek-V3.1-Terminus`, camada S 68.4%)

### Adicionado — Recursos TUI

- **Tecla `N` — Filtro de Origem/provedor**: percorre Todos → NIM → Groq → Cerebras → SambaNova → OpenRouter → Codestral → Hyperbolic → Scaleway → Google AI → Todos, espelhando como a tecla `T` percorre as camadas. O provedor ativo é mostrado como um selo no cabeçalho. O cabeçalho da coluna Origin agora lê `Origin(N)` e destaca-se em azul quando um filtro está ativo.
- **Tecla `C` — Ordenar por janela de contexto**: a ordenação por janela de contexto estava anteriormente na tecla `N`; movida para `C` (mnemônico: Context) para liberar a tecla `N` para o filtro de origem.
- **Tecla `K` — Overlay de Ajuda**: pressione `K` (ou `Esc`) para abrir/fechar uma referência completa de atalhos de teclado listando cada tecla e o que ela faz, renderizada no buffer de tela alternativa sem sair da TUI.
- **`Esc` fecha ajuda e configurações**: pressionar Escape agora dispensa tanto o overlay de ajuda `K` quanto a tela de configurações `P`. O overlay de ajuda intercepta o Esc antes do manipulador de configurações para que não haja conflito de teclas.

### Alterado — README & UI

- Selo de contagem de provedores atualizado: **4 → 9 provedores**
- Selo de contagem de modelos atualizado: **67 → 101 modelos**
- A seção de Requisitos lista todos os 9 provedores com suas URLs de cadastro.
- Bloco de cabeçalho do Discord substituído por um link simples `💬 Let's talk about the project on Discord`.
- Seção de Suporte reformatada: link de issues do GitHub + link do Discord em linhas separadas + link da documentação com aviso BETA em linha (`⚠️ free-coding-models is a BETA TUI — expect rough edges and occasional crashes`).
- Linha de dicas do rodapé atualizada: `T Tier • N Origin • … C` substitui o antigo `N` na dica de ordenação; `K Ajuda` adicionado.

### Técnico

- `sources.js`: 5 novos exports nomeados; objeto `sources` estendido para 9 entradas; JSDoc `@exports` atualizado.
- `lib/config.js`: `ENV_VARS` estendido com `openrouter`, `codestral`, `hyperbolic`, `scaleway`, `googleai`; comentário da estrutura da config JSDoc atualizado.
- `bin/free-coding-models.js`: assistente de primeira execução estendido para 9 provedores; `ENV_VAR_NAMES` estendido; blocos de provedores OpenCode/OpenCode-Desktop adicionados para todos os 5 novos provedores (todos usam `@ai-sdk/openai-compatible` + baseURL); estado `ORIGIN_CYCLE` + `originFilterMode`; a assinatura `renderTable` ganha o parâmetro `originFilterMode`; função `renderHelp()` adicionada; todos os locais de chamada de `renderTable` atualizados.

---

## 0.1.53

### Adicionado

- **SambaNova Cloud** como um novo provedor (teste gratuito de $5, 3 meses). 10 modelos de codificação: Qwen3 235B, DeepSeek R1 0528, DeepSeek V3.1, DeepSeek V3 0324, Llama 4 Maverick, GPT OSS 120B, Qwen3 32B, R1 Distill 70B, Llama 3.3 70B, Llama 3.1 8B. Endpoint compatível com OpenAI em `api.sambanova.ai`. Prefixo da chave: `sn-`
- **Cerebras**: Qwen3 235B (`qwen-3-235b-a22b`), GPT OSS 120B (`gpt-oss-120b`), Llama 3.1 8B (`llama3.1-8b`)
- **Groq**: Llama 3.1 8B (`llama-3.1-8b-instant`, 14.400 req/day)
- Integração total com OpenCode + OpenCode Desktop para SambaNova (bloco do provedor `@ai-sdk/openai-compatible` injetado automaticamente na seleção do modelo).
- SambaNova adicionada ao assistente de chave de API da primeira execução e à tela de Configurações (tecla `P`).

---

## 0.1.52

### Corrigido

- **Handoff do modelo OpenCode** (PR #14 por @whit3rabbit): as chaves de API de `~/.free-coding-models.json` não eram passadas para o processo filho do OpenCode, causando fallback silencioso para o modelo anterior. Também corrige incompatibilidades de ID de modelo da Groq (ex: `kimi-k2-instruct` → `kimi-k2-instruct-0905`) via um novo `OPENCODE_MODEL_MAP`.
- **Provedor nvidia do OpenClaw sem o array de modelos** (PR #13 por @whit3rabbit): `startOpenClaw()` criava o bloco do provedor nvidia sem a propriedade `models`, fazendo com que a validação do esquema Zod rejeitasse o config.

### Melhorado

- **Link do Discord no rodapé da TUI**: a URL de convite agora é exibida em texto simples em uma linha separada para que seja visível e possa ser copiada em terminais que não suportam links clicáveis.

---

## 0.1.51

### Corrigido

- **Modelos da Groq/Cerebras selecionados para o OpenCode não tinham bloco de provedor**: mesmo com o prefixo correto `groq/model-id`, o OpenCode não conseguia usar o modelo porque não existia nenhum bloco `provider.groq` no `opencode.json` — agora cria automaticamente o bloco do provedor (Groq: nativo com `apiKey: {env:GROQ_API_KEY}`; Cerebras: `@ai-sdk/openai-compatible` com baseURL) e registra o modelo em `provider.<key>.models`.

## 0.1.50

### Corrigido

- **Modelos da Groq/Cerebras selecionados para o OpenCode eram lançados como modelos NVIDIA**: o `providerKey` não era passado no `userSelected` ao pressionar Enter, fazendo com que todos os modelos fossem prefixados com `nvidia/` independentemente de seu provedor real — agora usa corretamente `groq/model-id` e `cerebras/model-id`.
- **`startOpenCode` e `startOpenCodeDesktop`**: ambas as funções agora lidam com todos os 3 provedores; Groq e Cerebras usam o suporte a provedores integrado do OpenCode (nenhum bloco de config personalizado é necessário, apenas as variáveis de ambiente `GROQ_API_KEY`/`CEREBRAS_API_KEY`); a NVIDIA mantém seu fluxo de configuração de provedor personalizado existente.

---

## 0.1.49

### Corrigido

- **Cerebras / Groq sem chave de API**: modelos estavam sendo pingados com a chave de reserva da NVIDIA, causando um `❌ 401` enganoso — agora faz o ping sem o cabeçalho de autenticação; o 401 é tratado como `🔑 NO KEY` (servidor acessível, latência mostrada de forma fraca).
- **Configurações: inserir uma chave de API não tinha efeito imediato**: após salvar uma chave e fechar as Configurações (Escape), modelos que estavam anteriormente no estado `noauth` agora são imediatamente re-pingados com a nova chave.

### Alterado

- O ping sem chave de API agora é sempre tentado — uma resposta 401 confirma que o servidor está ATIVO e mostra a latência real; `🔑 NO KEY` substitui o antigo erro enganoso `❌ 401`.

---

## 0.1.48

### Corrigido

- **Flag de CLI `--tier`**: `parseArgs()` nunca era chamado em `main()`, portanto `--tier S` era silenciosamente ignorado — agora está conectado e aplicado na inicialização da TUI (obrigado @whit3rabbit, PR #11).
- **Valor de `--tier` vazando para `apiKey`**: o loop for do `parseArgs()` estava capturando o valor da camada como a chave da API — corrigido pulando o argumento do valor após o `--tier`.
- **Ctrl+C não saindo**: o manipulador de teclas de ordenação estava interceptando todas as teclas pressionadas de uma única letra, incluindo as modificadas por ctrl — adicionada a proteção `!key.ctrl` para que o Ctrl+C chegue ao manipulador de saída (PR #11).

### Adicionado

- Teste verificando que o valor `--tier` não vaza para `apiKey` (63 testes no total).

---

## 0.1.47

### Corrigido

- **Flag de CLI `--tier`**: `parseArgs()` nunca era chamado em `main()`, portanto `--tier S` era silenciosamente ignorado — agora está conectado e aplicado na inicialização da TUI (obrigado @whit3rabbit, PR #11).
- **Valor de `--tier` vazando para `apiKey`**: o loop for do `parseArgs()` estava capturando o valor da camada como a chave da API — corrigido pulando o argumento do valor após o `--tier`.
- **Ctrl+C não saindo**: o manipulador de teclas de ordenação estava interceptando todas as teclas pressionadas de uma única letra, incluindo as modificadas por ctrl — adicionada a proteção `!key.ctrl` para que o Ctrl+C chegue ao manipulador de saída (PR #11).

### Adicionado

- Teste verificando que o valor `--tier` não vaza para `apiKey` (63 testes no total).

---

## 0.1.46

### Corrigido

- **Notificação do Discord**: corrigido o erro ECONNRESET — drena o corpo da resposta com `res.resume()` e chama `process.exit(0)` imediatamente após o sucesso para que o processo Node feche de forma limpa.

### Alterado

- **Link do Discord**: Atualizada a URL do convite para `https://discord.gg/5MbTnDC3Md` em todos os lugares (README, rodapé da TUI).

---

## 0.1.45

### Corrigido

- **Notificação do Discord**: corrigida a falha no fluxo de trabalho do GitHub Actions (contexto de segredos não permitido em condições `if` de etapa — agora tratado diretamente no script Node).

---

## 0.1.44

### Adicionado

- **Suporte a múltiplos provedores** — Groq (6 modelos) e Cerebras (3 modelos) adicionados juntamente com o NVIDIA NIM, totalizando 53 modelos.
- **Assistente de múltiplos provedores na primeira execução** — percorre todos os 3 provedores (NIM, Groq, Cerebras) no primeiro lançamento; cada um é opcional, pressione Enter para pular; requer pelo menos uma chave.
- **Tela de Configurações (tecla `P`)** — Novo overlay na TUI para gerenciar chaves de API por provedor, habilitar/desabilitar provedores e testar chaves com um ping ao vivo.
- **`lib/config.js`** — Novo sistema de configuração JSON (`~/.free-coding-models.json`) substituindo o antigo arquivo de texto simples.
  - Auto-migra o antigo `~/.free-coding-models` (chave nvidia simples) na primeira execução.
  - Armazena as chaves por provedor + o estado habilitado/desabilitado por provedor.
  - Variáveis de ambiente `NVIDIA_API_KEY`, `GROQ_API_KEY`, `CEREBRAS_API_KEY` sobrescrevem a config.
- **URLs de ping por provedor** — `ping()` agora aceita uma URL de endpoint explícita; cada provedor tem seu próprio endpoint de API em `sources.js`.
- **Nome do provedor na coluna Origem** — Mostra `NIM` / `Groq` / `Cerebras` em vez de sempre `NIM`.

### Alterado

- O array plano `MODELS` agora inclui `providerKey` como o 6º elemento.
- O estado inicial filtra modelos de provedores desativados; reconstrói ao fechar as configurações.
- Caminho do arquivo de configuração alterado de `~/.free-coding-models` para `~/.free-coding-models.json` (a migração é automática).

---

## 0.1.41 — 22-02-2026

### Alterado

- **Auditoria de dados do sources.js** — verificadas e corrigidas as pontuações SWE-bench, camadas e janelas de contexto em todos os modelos NIM:
  - Devstral 2 123B: `S, 62,0%, 128k` → `S+, 72,2%, 256k` (anúncio oficial da Mistral)
  - Mistral Large 675B: ctx `128k` → `256k`
  - QwQ 32B: ctx `32k` → `131k`
  - Llama 4 Maverick: ctx `128k` → `1M` (NVIDIA NIM confirmado)
  - Llama 4 Scout: ctx `128k` → `10M` (NVIDIA NIM confirmado)
  - GPT OSS 20B: ctx `32k` → `128k`

---

## 0.1.38 — 22-02-2026

### Corrigido

- **Integração OpenCode multiplataforma**: corrigidos problemas de instalação do OpenCode CLI e Desktop no Windows e Linux.
  - **Windows**: corrigido o caminho da config para usar %APPDATA%\opencode\opencode.json com fallback para ~/.config.
  - **Linux**: adicionado suporte para snap, flatpak e xdg-open para lançar o OpenCode Desktop.
  - **Todas as plataformas**: detecta corretamente o SO e usa os comandos e caminhos corretos.
  - **OpenCode Desktop**: comandos de lançamento específicos da plataforma (macOS: `open -a`, Windows: `start`, Linux: múltiplos métodos).

---

## 0.1.37 — 22-02-2026

### Adicionado

- **Auto-atualização com fallback via sudo**: quando a atualização do npm falha devido a permissões, tenta novamente automaticamente com sudo para completar a atualização.

---

## 0.1.36 — 22-02-2026

### Adicionado

- **Coluna SWE-bench Verified**: mostra as pontuações reais do SWE-bench Verified para todos os 44 modelos a partir de benchmarks oficiais.
- **Atalhos de teclado codificados por cores**: a primeira letra de cada cabeçalho de coluna colorida em amarelo para indicar a tecla de ordenação.
- **Coração e Café no rodapé**: "Made with 💖 & ☕ by vava-nessa"

### Alterado

- **Organização das colunas**: colunas reordenadas para um fluxo lógico melhor: Rank / Camada / SWE% / Modelo / Origem / Último Ping / Média de Ping / Saúde / Veredito / Up%.
- **Coluna de Saúde**: renomeada de "Status" para "Health" com a tecla H para ordenação.
- **Ordenação SWE-bench**: a tecla S agora ordena pela pontuação SWE-bench.
- **Atalho do último ping**: tecla L (em vez de P) para ordenar pelo último ping.
- **Nome da fonte**: simplificado de "NVIDIA NIM" para "NIM".

### Corrigido

- **Alinhamento dos cabeçalhos das colunas**: corrigido o desalinhamento causado pelos códigos de cores ANSI nos cabeçalhos.
- **Link do Discord**: Atualizado para o link de convite permanente https://discord.gg/WKA3TwYVuZ.

---

## 0.1.35 — 22-02-2026

### Alterado

- **Reorganização das colunas**: colunas reordenadas para um fluxo lógico melhor: Rank / Camada / SWE% / Modelo / Origem / Último Ping / Média de Ping / Saúde / Veredito / Up%.

---

## 0.1.34 — 22-02-2026

### Alterado

- **Condição renomeada para Saúde**: Renomeada a coluna "Condition" para "Health" para melhor clareza.
- **Atualização do atalho de teclado**: a tecla H agora ordena pela Saúde (em vez de C para Condition).

---

## 0.1.33 — 22-02-2026

### Corrigido

- **Alinhamento dos cabeçalhos das colunas**: corrigido o problema de desalinhamento dos cabeçalhos das colunas causado pelos códigos de cores ANSI que interferiam no preenchimento do texto.

---

## 0.1.32 — 22-02-2026

### Alterado

- **Melhorias nos cabeçalhos das colunas**: corrigidos problemas de alinhamento das colunas para uma melhor aparência visual.
- **Status renomeado para Condição**: coluna "Status" renomeada para "Condition" para maior clareza.
- **Atualizações dos atalhos de teclado**: a tecla S agora ordena pela pontuação SWE-bench, a tecla C ordena por Condição.
- **Atualização do texto do Discord no rodapé**: alterado "Join our Discord!" para "Join Free-Coding-Models Discord!".

---

## 0.1.31 — 22-02-2026

### Adicionado

- **Coluna SWE-bench**: adicionada nova coluna de pontuação SWE-bench Verified mostrando o desempenho de codificação para cada modelo.
- **Cabeçalhos de colunas codificados por cores**: a primeira letra de cada cabeçalho de coluna agora é colorida (amarelo) para indicar o atalho de teclado para ordenação.
- **Melhorias nos atalhos de teclado**: alterado de P para L para a ordenação por último ping, adicionado E para ordenação pelo SWE-bench.

### Alterado

- **Simplificação do nome da fonte**: renomeado "NVIDIA NIM" para "NIM" em toda a base de código.
- **Link do Discord aprimorado no rodapé**: o link do Discord agora é exibido na cor ciano brilhante com o indicador "(link fixed)".

---

## 0.1.29 — 22-02-2026

### Corrigido

- **Correção do link do Discord**: atualizadas todas as URLs de convite do Discord para usar o link permanente https://discord.gg/WKA3TwYVuZ.

---

## 0.1.28 — 22-02-2026

### Adicionado

- **Emojis no rodapé**: adicionado o emoji 💬 antes do link do Discord e o emoji ⭐ antes do link do GitHub para um melhor apelo visual.

---

## 0.1.27 — 22-02-2026

### Alterado

- **Redesign do rodapé**: todos os links agora em uma linha com texto clicável: "Join our Discord!" e "Read the docs on GitHub".
- **UX aprimorada**: os links usam o mesmo formato clicável do nome do autor para uma experiência de usuário consistente.

---

## 0.1.26 — 22-02-2026

### Alterado

- **Melhorias no rodapé**: substituído "Repository GitHub" por "GitHub", "love" pelo emoji 💖 e simplificado o texto do Discord.
- **Aprimoramento do README**: adicionada seção de link do GitHub abaixo do convite do Discord.

---

## 0.1.25 — 22-02-2026

### Adicionado

- **Link da comunidade do Discord**: adicionado convite do Discord ao README e ao rodapé da TUI.
- **Layout de rodapé aprimorado**: rodapé melhorado com layout multi-linha mostrando o repositório GitHub e os links do Discord.
- **Nome do autor clicável**: "vava-nessa" agora é clicável no terminal (abre o perfil do GitHub).
- **Automação das notas de lançamento**: o GitHub Actions agora usa o conteúdo do CHANGELOG.md para as notas de lançamento em vez das notas geradas automaticamente.

### Alterado

- **Sistema de filtragem por camada**: substituídas as teclas E/D pela tecla T que percorre os filtros de camada: tudo → S+/S → A+/A/A- → B+/B → C → tudo.
- **Texto do rodapé**: "Made with love by vava-nessa" com links clicáveis.

### Corrigido

- **Fluxo de trabalho de lançamento**: as versões do GitHub agora exibem o conteúdo adequado do changelog em vez de resumos genéricos de commits.

---

## 0.1.24 — 22-02-2026

### Corrigido

- **Rolagem de viewport para transbordo da TUI**: corrigidos problemas do Ghostty e de terminais estreitos onde o conteúdo rolava além da tela alternativa.
- **Envolvimento de linha do terminal**: as linhas largas agora são cortadas na borda do terminal em vez de continuarem na próxima linha.
- **Poluição do scrollback**: substituído `\x1b[2J` por `\x1b[H` + `\x1b[K` por linha para evitar problemas de scrollback no Ghostty.
- **Cálculo da viewport**: adicionada rolagem inteligente com indicadores "N more above/below" quando os modelos excedem a altura da tela.
- **Ajuste do deslocamento da rolagem**: o cursor permanece dentro da janela visível durante a navegação e o redimensionamento do terminal.

### Alterado

- **DECAWM desativado**: desativado o envolver automático na tela alternativa para evitar que a altura da linha dobre.
- **Manipulação de redimensionamento de terminal**: a viewport se ajusta automaticamente quando o tamanho do terminal muda.

---

## 0.1.23 — 22-02-2026

### Refatorado

- **Menu de inicialização removido**: não há mais menu de seleção de modo bloqueante na inicialização.
- **Padrão para OpenCode CLI**: o app inicia diretamente no modo CLI quando nenhuma flag é fornecida.
- **Alternar modo na TUI**: adicionada a tecla Z para percorrer entre CLI → Desktop → OpenClaw → CLI.
- **Changelogs do GitHub**: a opção "Read Changelogs" agora abre a URL do GitHub em vez do arquivo local.
- **Auto-atualização por padrão**: quando uma nova versão está disponível sem flags, auto-atualiza e reinicia.
- **Menu de atualização centralizado**: a notificação de atualização aparece apenas quando necessário, com um layout centralizado e limpo.

### Alterado

- **Exibição do cabeçalho**: mostra `[💻 CLI] (Z para alternar)` com a dica de alternar modo.
- **Instruções do rodapé**: Adicionado "M Mode" aos atalhos de teclado.
- **Fluxo de atualização**: flags (`--opencode`, etc.) ainda mostram o menu de atualização por compatibilidade.

---

## 0.1.22 — 22-02-2026

### Alterado

- **Changelogs locais**: a opção do menu "Read Changelogs" agora abre o arquivo `CHANGELOG.md` local em vez dos lançamentos do GitHub.

---

## 0.1.21 — 22-02-2026

### Refatorado

- **Arquitetura de filtragem de camada simplificada**: substituída a recriação complexa de objetos por um sistema simples de flag `hidden`.
- **Flags como atalhos**: `--tier S` agora apenas define o estado inicial em vez de bloquear a filtragem dinâmica.
- **Filtragem dinâmica preservada**: as teclas E/D funcionam perfeitamente mesmo ao iniciar com a flag `--tier`.

### Corrigido

- **Bug do loop de ping**: corrigido o problema onde os modelos filtrados não eram pingados devido ao uso do array de resultados errado.
- **Bug do ping inicial**: corrigido o problema onde o ping inicial usava o array de resultados errado.

---

## 0.1.20 — 22-02-2026

### Adicionado

- **Filtragem de camada dinâmica**: use as teclas E/D para filtrar modelos por camada durante o tempo de execução.
- Selo de filtro de camada mostrado no cabeçalho (ex: `[Tier S]`).
- Tecla E eleva o filtro (mostra menos modelos, de camadas superiores).
- Tecla D desce o filtro (mostra mais modelos, de camadas inferiores).
- Preserva o histórico de ping ao alterar filtros.

### Corrigido

- **Erro 401 com a flag --tier**: corrigido o problema onde usar apenas `--tier` mostraria o menu de seleção em vez de prosseguir diretamente para a TUI.
- Manuseio de combinação de flags aprimorado para uma melhor experiência do usuário.

---

## 0.1.16

### Adicionado

- Suporte ao OpenCode Desktop: nova flag `--opencode-desktop` e opção de menu para definir o modelo e abrir o app Desktop.
- Opção de menu "Read Changelogs" quando uma atualização está disponível (abre a página de lançamentos do GitHub).
- Função `startOpenCodeDesktop()` — mesma lógica de config do CLI, lança via `open -a OpenCode`.

### Alterado

- Menu de inicialização: "OpenCode" renomeado para "OpenCode CLI", nova entrada "OpenCode Desktop" adicionada.
- Selo de modo TUI: mostra `[💻 CLI]` ou `[🖥 Desktop]` ou `[🦞 OpenClaw]`.
- A dica de ação do rodapé se adapta ao modo desktop (`Enter→OpenDesktop`).

---

## 0.1.12 — 22-02-2026

### Adicionado

- Suíte de testes unitários: 59 testes em 11 suítes usando `node:test` (zero dependências).
- Os testes cobrem: integridade de dados das fontes, lógica principal (getAvg, getVerdict, getUptime, filterByTier, sortResults, findBestModel), parsing de argumentos CLI, sanidade do package.json.
- `lib/utils.js`: funções de lógica pura extraídas do CLI monolítico para testabilidade.
- Script `pnpm test` no package.json.

### Corrigido

- Fluxo de trabalho de lançamento do GitHub Actions: removido o loop quebrado do `npm version patch`, adicionada detecção de versão via tags git.
- O GitHub Actions agora cria um GitHub Release com notas geradas automaticamente para cada nova versão.

### Alterado

- AGENTS.md atualizado com fluxo de trabalho de test-first: agentes devem executar `pnpm test` antes de `pnpm start`.

---

## 0.1.9 — 22-02-2026

### Corrigido

- **Spawn ENOENT do OpenCode**: use `shell: true` ao fazer o spawn do `opencode` para que o comando seja resolvido corretamente no Windows (wrappers `.cmd`/`.bat`). Adicionada mensagem de erro amigável quando o `opencode` não está instalado.
### Adicionado

- Aviso de atualização disponível: mensagem vermelha mostrada acima do menu de seleção quando uma nova versão do npm existe.
- Escolha de menu "Update now" no modo de inicialização para instalar a versão mais recente.

---

## 0.1.4 — 22-02-2026

### Corrigido

- **Estrutura de config do OpenClaw**: `providers` foi escrito incorretamente na raiz da config. Movido para `models.providers` conforme a documentação oficial do OpenClaw (`docs.openclaw.ai/providers/nvidia`).
- **Armazenamento da chave de API do OpenClaw**: removido `apiKey` do bloco do provedor (não é um campo reconhecido). A chave de API agora é armazenada em `env.NVIDIA_API_KEY` na config.
- **Array de modelos do OpenClaw**: removido o array `models: []` do bloco do provedor (formato OpenCode, não é válido no OpenClaw).
- **Comando CLI `openclaw restart` não existe**: substituída a dica pelos comandos corretos — `openclaw models set` / `openclaw configure`. O gateway recarrega automaticamente com as mudanças no arquivo de configuração.
- **Modelo OpenClaw não permitido**: o modelo deve ser explicitamente listado na allowlist `agents.defaults.models` — sem isso, o OpenClaw rejeita o modelo com "not allowed", mesmo quando definido como primário.
- **README**: seção de integração do OpenClaw atualizada com a estrutura JSON correta e comandos CLI corretos.

---

## 0.1.3 — 22-02-2026

### Adicionado

- Integração com OpenClaw: define o modelo NIM selecionado como provedor padrão em `~/.openclaw/openclaw.json`.
- Menu de modo de inicialização (nenhuma flag necessária): escolha interativa entre OpenCode e OpenClaw no lançamento.
- Flag `--openclaw`: pula o menu, vai direto para o modo OpenClaw.
- Flag `--tier`: filtra os modelos pela letra da camada (S, A, B, C).
- Selos de camada mostrados ao lado dos nomes dos modelos na TUI.
- 44 modelos listados, classificados pelo benchmark Aider Polyglot.

### Corrigido

- Permissões de CI para git push no fluxo de trabalho de lançamento.

---

## 0.1.2 — 22-02-2026

### Adicionado

- Flag `--fiable`: analisa por 10 segundos, envia o modelo único mais confiável como `provider/model_id`.
- Flag `--best`: mostra apenas os modelos de camada superior (A+, S, S+).
- Flag `--opencode`: modo OpenCode explícito.
- Ponto de entrada CLI refatorado, manuseio de flags mais limpo.
- Fluxo de trabalho de lançamento atualizado.

---

## 0.1.1 — 21-02-2026

### Adicionado

- Modo de monitoramento contínuo: re-pinga todos os modelos a cada 2 segundos para sempre.
- Médias móveis calculadas a partir de todos os pings bem-sucedidos desde o início.
- Rastreamento de porcentagem de uptime por modelo.
- Intervalo de ping dinâmico: tecla W para acelerar, tecla X para desacelerar.
- Colunas ordenáveis: teclas R/T/O/M/P/A/S/V/U.
- Coluna Veredito com classificação de qualidade por modelo.
- Seleção interativa de modelo com as teclas de seta + Enter.
- Integração com OpenCode: detecta automaticamente a configuração NIM, define o modelo como padrão, lança o OpenCode.
- `sources.js`: arquitetura extensível para adicionar novos provedores.
- GIF de demonstração adicionado ao README.
- CLI renomeado para `free-coding-models`.

---

## 0.1.0 — 21-02-2026

### Adicionado

- Lançamento inicial como `nimping` e depois renomeado para `free-coding-models`.
- Pings paralelos de modelos de codificação NVIDIA NIM via `fetch` nativo.
- Tabela de terminal em tempo real com exibição de latência.
- Buffer de tela alternativa (sem poluição de scrollback).
- Top 3 modelos mais rápidos destacados com medalhas 🥇🥈🥉.
- Banner ASCII e UI limpa.
- Instalador do OpenCode e seletor interativo de modelos.
- Fluxo de trabalho de publicação no npm via GitHub Actions.
