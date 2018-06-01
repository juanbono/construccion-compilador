# construccion-compilador
Construccion de un compilador de Lisp


# Material de consulta

- Libro: [Structure and Interpretation of Computer Programs](https://mitpress.mit.edu/sites/default/files/sicp/full-text/book/book-Z-H-38.html#%_index_start)
- Video: [Structure and Interpretaion of Computer Programs](https://www.youtube.com/watch?v=2Op3QLzMgSY)
- [Racket Lisp](https://racket-lang.org/)
- [Implementing a Lisp](http://wiki.c2.com/?ImplementingLisp)
- [Implementing (the original) Lisp in Python](http://kjetilvalle.com/posts/original-lisp.html)
- [LALRPOP parser generator](https://github.com/lalrpop/lalrpop)
- [LALRPOP quick start](http://lalrpop.github.io/lalrpop/quick_start_guide.html)
- [LALRPOP book](http://lalrpop.github.io/lalrpop/tutorial)
- [Lisp implementada en Rust 1](https://github.com/swgillespie/rust-lisp)
- [Implementing Lisp in Java](http://www.cis.upenn.edu/~matuszek/cit594-2003/Assignments/03-implementing-lisp.html)
- [SO de gramatica de Lisp](https://stackoverflow.com/questions/517113/lisp-grammar-in-yacc)
- [Implementing Lisp and Maxwell equations](http://www.righto.com/2008/07/maxwells-equations-of-software-examined.html)
- [Implementing Lisp by Peter Norvig](http://norvig.com/lispy.html)

## Objetivos

- Construir un lenguage de programacion real y practico
- Ganar experiencia y conocimiento y un mayor entendimiento sobre la construccion de compiladores individual y compartida
- Crear un entregable con una parte teorica y otra practica que nos permita distribuir nuestro conocimiento adquirido en distintos contextos.

## Herramientas
- Rust como lenguage de programacion porque nos da muy buena velocidad y muchas garantias en tiempo de compilacion

## Marco teorico

### Lexer

Utilizaremos un lexer basado en expresiones regulares provistas por el standard library de Rust

TODO:
- teoria de reg exp
- teoria de automatas


### Gramatica / Sintaxis

TODO
- definir la sintaxis exacta (probablemente algo parecido a Racket o Clojure)

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

## Preguntas
- Podemos parsear Lisp con LR1?
- Como implementar reader-macros y otras caracteristicas especiales de Lisp?



### TODO
- cargo fmt
- clippy
- tests
