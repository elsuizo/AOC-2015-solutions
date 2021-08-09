# Dia 7

Este año Santa le compro al pequeño Boby un conjunto de cables y unas
compuertas logicas. Desafortunadamente el chico es muy pequenio y no puede
solo, por eso necesita de nuestra ayuda.

Cada cable tiene un identificador(a lo sumo dos letras) y puede llevar una
senial de 16bits. La senial es provista a cada cable por una compuerta logica,
otro cable o por un valor especifico. Cada cable solo puede obtener su senial
de una sola fuente pero puede proveer la suya a varias compuertas, una
compuerta solo emite un resultado cuando todas las seniales que entran en ellas
tienen una senial

Las instrucciones que nos dan describen como conectar los cables y las
compuertas por ejemplo: `x AND y -> z` significa que tenemos que conectar los
cables `x` e `y` a una compuerta `AND` y la salida asignarsela a el cable `z`

Por ejemplo:

`123 -> x` Significa que la senial `123` es asignada a el cable `x`

Por ejemplo con las siguientes instrucciones:

```txt
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
```
Despues de realizarlas las seniales tienen los siguientes valores:

```txt
d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456
```

Con las instrucciones que le dan al ninio (que es nuestra `input`) cual es el valor
que tiene el cable `a`
