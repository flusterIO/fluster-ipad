# Conundrum Database

The aim with this is to basically be the datalayer for your brain. Other apps can integrate with it however they like, but this will establish a completely local vector database on your machine, and along with the Conundrum server package, and whatever front-end application you choose to use or build, this establishes a single source of truth for the data.

There's room for everything academic, in a way that's both tabular and vector oriented.

- Assigments with:
  - Alerts
  - Milestones
  - Status's that replicate project management kanban boards.
- Results
  - These can be used for either academic results, like the result on a test or an assigment, or the result of an experiment with the way that the data is structured. AI should be able to happily read from both cases, and be able to tell the difference.
- Conundrum/Markdown content
  - Of course it's going to support conundrum, but regular markdown will work too. There's even a field to specify that it is just regular markdown, and all conundrum content is parsed to regular markdown before being passed to AI... until I can find the time and money to properly fine-tune a model.
- Tags/Topics/Subjects/Citations
  - All the good stuff that links your notes together. Each app can implement it the way the developer chooses, but your database supports all three (So does Fluster :)).
