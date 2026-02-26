# Instruções para Agentes

## Testes Pós-Recurso

Após concluir qualquer recurso ou correção, o agente DEVE:

1. Executar `pnpm test` para verificar se todos os testes unitários passam (62 testes em 11 suítes)
2. Se algum teste falhar, corrija o problema imediatamente
3. Execute novamente `pnpm test` até que todos os testes passem
4. Execute `pnpm start` para verificar se não há erros em tempo de execução
5. Se houver erros, corrija-os imediatamente
6. Execute novamente `pnpm start` até que todos os erros sejam resolvidos
7. Só então considere a tarefa concluída

Isso garante que a base de código permaneça em um estado funcional em todos os momentos.

## Processo de Lançamento (OBRIGATÓRIO)

Ao lançar uma nova versão, siga exatamente este processo:

1. **Verificação de Versão**: Verifique se a versão já existe com `git log --oneline | grep "^[a-f0-9]\+ [0-9]"`
2. **Incremento de Versão**: Atualize a versão no `package.json` (ex: `0.1.16` → `0.1.17`)
3. **Commit de TODOS os Arquivos Alterados**: `git add . && git commit -m "0.1.17"`
   - Sempre faça o commit apenas com o número da versão como mensagem (ex: "0.1.17")
   - Inclua TODOS os arquivos modificados no commit (bin/, lib/, test/, README.md, CHANGELOG.md, etc.)
4. **Push**: `git push origin main` — O GitHub Actions publicará automaticamente no npm
5. **Aguardar Publicação no npm**:
   ```bash
   for i in $(seq 1 30); do sleep 10; v=$(npm view free-coding-models version 2>/dev/null); echo "Attempt $i: npm version = $v"; if [ "$v" = "0.1.17" ]; then echo "✅ published!"; break; fi; done
   ```
5. **Instalar e Verificar**: `npm install -g free-coding-models@0.1.17`
6. **Testar Binário**: `free-coding-models --help` (ou qualquer outro comando para verificar se funciona)
7. **Somente quando a versão instalada globalmente via npm funcionar → o lançamento é confirmado**

**Por quê:** Um `npm install -g .` local pode mascarar problemas porque ele cria um link simbólico para o repositório. O pacote npm real é um tarball construído a partir do campo `files` — apenas uma instalação real do npm pegará arquivos ausentes.

## Verificação npm em Mundo Real (OBRIGATÓRIO para cada correção/recurso)

**Nunca confie apenas em testes locais.** `pnpm start` roda a partir do repositório e não pegará arquivos ausentes no pacote publicado. Sempre execute a verificação npm completa:

1. Incremente a versão no `package.json` (ex: `0.1.14` → `0.1.15`)
2. Commit e push para `main` — O GitHub Actions publica automaticamente no npm
3. Aguarde a nova versão aparecer no npm:
   ```bash
   # Poll até que o npm tenha a nova versão
   for i in $(seq 1 30); do sleep 10; v=$(npm view free-coding-models version 2>/dev/null); echo "Attempt $i: npm version = $v"; if [ "$v" = "NEW_VERSION" ]; then echo "✅ published!"; break; fi; done
   ```
4. Instale a versão publicada globalmente:
   ```bash
   npm install -g free-coding-models@NEW_VERSION
   ```
5. Execute o binário global e verifique se funciona:
   ```bash
   free-coding-models
   ```
6. Somente se a versão instalada globalmente via npm funcionar → a correção é confirmada

**Por quê:** Um `npm install -g .` local pode mascarar problemas porque ele cria um link simbólico para o repositório. O pacote npm real é um tarball construído a partir do campo `files` — se algo estiver faltando lá, apenas uma instalação real do npm pegará.

## Arquitetura de Teste

- Os testes residem em `test/test.js` usando o `node:test` integrado do Node.js + `node:assert` (zero dependências)
- Funções de lógica pura estão em `lib/utils.js` (extraídas do CLI principal para testabilidade)
- O CLI principal (`bin/free-coding-models.js`) importa de `lib/utils.js`
- Se você adicionar nova lógica pura (cálculos, parsing, filtragem), adicione-a a `lib/utils.js` e escreva testes
- Se você modificar lógica existente em `lib/utils.js`, atualize os testes correspondentes

### O que é testado:
- **Integridade de dados de sources.js** — estrutura do modelo, camadas válidas, sem duplicatas, consistência de contagem
- **Lógica principal** — getAvg, getVerdict, getUptime, filterByTier, sortResults, findBestModel
- **Parsing de argumentos CLI** — todas as flags (--best, --fiable, --opencode, --openclaw, --tier)
- **Sanidade do pacote** — campos do package.json, entrada bin existe, shebang, imports ESM

## Contribuidores do GitHub

Quando novos PRs forem mesclados, adicione o identificador do GitHub do contribuidor ao rodapé em `bin/free-coding-models.js` (a linha `Contributors:` perto da linha 775), separados por espaços. Também atualize esta lista:

- @whit3rabbit

## Registro de Alterações (OBRIGATÓRIO)

**⚠️ CRÍTICO:** Após cada sessão de desenvolvimento (recurso, correção, refatoração), adicione uma entrada sucinta ao `CHANGELOG.md` ANTES de fazer o push:

- Use a versão atual do `package.json`
- Adicione sob o cabeçalho da versão correspondente (ou crie um novo se a versão foi incrementada)
- Se a versão atual já estiver publicada, **não** adicione novas entradas sob essa versão publicada: crie o **próximo** cabeçalho de versão (exemplo: `0.1.63` já publicada → documente o novo trabalho sob `0.1.64`)
- Liste as alterações sob `### Added`, `### Fixed` ou `### Changed` conforme apropriado
- Mantenha as entradas curtas — uma linha por alteração é suficiente
- Mantenha a seção de lançamento superior limpa e voltada para o usuário para que possa ser reutilizada diretamente na tela de notas de lançamento do GitHub (balas claras, sem ruído interno)
- Inclua TODAS as alterações feitas durante a sessão
- Atualize o CHANGELOG.md ANTES de fazer o commit e o push

**Por que isso é crítico:** O registro de alterações é o único registro histórico do que foi alterado em cada versão. Sem ele, os usuários não conseguem entender o que mudou entre as versões.
