# Dia 8

El espacio en el trineo es un poco limitado este año y entonces Santa va a
brindarnos su lista como una copia digital. El necesita saber cuanto espacio le
va a tomar cuando los acomode.

Es comun en muchos lenguajes de programacion de proveer una manera de escapar a
caracteres especiales en strings. Por ejemplo `C`, `Javascript`, `Perl`,
`Python` y tambien `PHP` manejan los caracteres especiales de manera similar

Sin embargo es importante darse cuenta las diferencias entre el numero de
caracteres en la representacion de codigo de un literal de string y el numero de
caracteres en memoria de la string en si misma

Por ejemplo:

 - `""` tiene 2 caracteres de codigo (las dos doble quote) pero el string
  contiene cero characteres
 - `"abc"` tiene 5 caracteres de codigo, pero tiene 3 caracteres en el string de
   datos
 - `"aaa\"aaa"` tiene 10 caracteres de codigo, pero el string en si mismo
 contiene seis `a` pero ademas tenemos una que esta "escapada" que es el quote
 o sea que tenemos 7 characteres de datos
 - `"\x27"` tiene 6 caracteres de codigo, pero el string en si mismo contiene
 solo un `-` como apostrofe (') escapado usando la notacion hexa

Santa tiene una lista como file que contiene muchos strings literales, uno en
cada linea del file. Las unicas secuencias de escape usadas son `\\` (que
representa un simple \) `\"` (que representa un simple caracter de double quote)
y `\x` mas dos numeros hexa (que representa un caracter simple con un codigo
ascii)

Ignorando los espacios en blanco en el archivo, cual es el numero de caracteres
de codigo para los strings literales menos el numero de caracteres en memoria
para los valores de los strings en todo el archivo

Por ejemplo para los cuatro strings anteriores, el total de numeros de
caracteres en el string de codigo (2 + 5 + 10 + 6 = 23) menos el total de numero
de caracteres en memoria para valores de strings (0 + 3 + 7 + 1 = 11) entonces
la respuesta es 23 - 11 = 12
