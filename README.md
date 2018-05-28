# construccion-compilador
Construccion de un compilador de Lisp


# Material de consulta

- Libro: [Structure and Interpretation of Computer Programs](https://mitpress.mit.edu/sites/default/files/sicp/full-text/book/book-Z-H-38.html#%_index_start)
- Video: [Structure and Interpretaion of Computer Programs](https://www.youtube.com/watch?v=2Op3QLzMgSY)


## Marco teorico

### Lexer

Utilizaremos un lexer basado en expresiones regulares provistas por el standard library de Rust

TODO:
- teoria de reg exp
- teoria de automatas

### Parser

Utilizaremos un [parser generator](https://github.com/lalrpop/lalrpop) que genera parsers LR1 escrito en Rust

TODO
- teoria de parsers
- teoria de LR1

### Modelo de memoria

TODO
- a definir

### Interprete Metacircular

Utilizaremos la tecnica detallada en SCIP en donde recursivamente las expresiones y declaraciones del codigo fuente
se van simplificando hasta llegar a primitivas basicas, efectivamente reduciendo el codigo original a un subconjunto
del lenguage original, mas simple, que el compilador finalmente pueda entender y procesar.



