# Modo sala de aula

O KataBox pode distribuir tarefas e recolher resultados sem servidor nem contas. Os dois
viajam como arquivos que você compartilha como preferir (e-mail, uma pasta
compartilhada, um pen drive).

**Não é à prova de trapaça.** O KataBox funciona offline no computador do aluno, então
quem quiser pode alterar seus dados. O arquivo de resultados traz uma cadeia de hashes
que mostra edições *casuais* (um veredito alterado num editor de texto), mas qualquer
pessoa pode recalculá-la. Use-o para praticar e dar retorno, não para notas decisivas.

## Para professores: criar uma tarefa
1. Escreva `assignment.toml`:

   ```toml
   title = "Semana 3: dois ponteiros"
   due = "2026-12-01"                    # AAAA-MM-DD
   problems = ["budget-pair", "two-sum"] # problemas do seu pacote ou ids embutidos
   languages = ["python", "typescript"]  # python, typescript, php, rust
   time_limit_min = 90                   # opcional, de 5 a 600 minutos a partir de Começar
   hints = false                         # opcional; false bloqueia dicas e o editorial
   ```

2. Escreva `pack.toml` (veja "Pacotes de problemas" no guia do usuário) e gere o pacote.
   Suas próprias pastas de problemas são opcionais; uma tarefa só com problemas embutidos
   não precisa de nenhuma:

   ```sh
   kb-pack pack meu-problema-1 meu-problema-2 --manifest pack.toml --assignment assignment.toml --out semana3.kbpack
   ```

3. Envie `semana3.kbpack` aos seus alunos.

## Para alunos
1. **Configurações → Pacotes → Importar um pacote…**, confira a prévia (ela mostra a
   tarefa), instale e reinicie.
2. Abra **Sala de aula**. Leia as regras e clique em **Começar** quando estiver pronto.
   Enquanto ela estiver em andamento, os problemas aceitam só as linguagens indicadas e,
   com as dicas desligadas, as dicas, o editorial, os envios anteriores e o Histórico
   ficam bloqueados. Só contam os envios feitos enquanto ela está em andamento. Ela termina
   quando você clica em **Terminar agora** ou quando o tempo acaba.
3. **Exportar resultados**: digite seu nome, escolha se quer incluir seu código, leia a
   lista exata do que o arquivo contém e salve. Envie esse arquivo ao seu professor.

## Para professores: recolher resultados
1. Abra **Sala de aula → Resultados da turma → Importar arquivos de resultados…** e
   escolha todos os arquivos recebidos (dá para escolher vários de uma vez). Cada arquivo
   é verificado: um arquivo corrompido ou estranho é recusado com o motivo, o mesmo
   arquivo duas vezes é importado uma vez só, e um arquivo editado depois de exportado é
   mantido e marcado como **Editado depois de exportado**.
2. A tabela mostra uma linha por aluno e uma coluna por problema.
3. **Exportar CSV…** grava uma linha por aluno e problema. Células que poderiam começar
   uma fórmula de planilha recebem um `'` na frente.

> A interface em português chega numa etapa seguinte; até lá, os nomes dos botões aparecem
> em inglês ou espanhol, conforme o idioma escolhido.
