# Dia7
## Parte1
Este año Santa le compro al pequeño Boby un circuito logico con cables y compuertas
logicas, pero es muy pequeño y necesita ayuda para armarlo.

Cada cable tiene un identificador (algunas letras en minusculas) y esta puede llevar
una señal de 16 bits. Una señal es provista para cada uno de los cables por una compuerta,
otro cable o por un valor especifico. Cada señal puede solo tener una señal de una
sola fuente, pero puede proveer su señal a muchas otras. Una compuerta provee una
señal solo cuando todas sus entradas tienen una señal.

Las instrucciones que nos dan describen como conectar las partes, por ejemplo:

`x AND y -> z` significa que tenemos que conectar el cable `x` con y el `y` a una
compuerta `AND` y conectar su salida a el cable `z`

Por ejemplo:

- `123 -> x`: Significa que la señal `123` es provista al cable `x`

Otras posibles compuertas incluyen `OR` `RSHIFT` `LSHIFT` `NOT` `AND`

Por ejemplo con el siguiente circuito:

```text
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
```
Obtenemos las siguientes señales:

```text
d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456
```

Con las instrucciones que nos dan(el input del puzzle) cual es la señal que contiene
el cable `a` despues de armar el circuito???

## Parte 2

Ahora tenemos que tomar la senial que calculamos en la parte 1, sobreescribir
la senial `b` con ese valor y resetear los otros valores de los otros cables
(incluyendo a `a`). Cual es la senial que tiene `a` ahora???
