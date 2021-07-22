# dia5

## parte 1

Santa necesita saber cual de las strings que le pasaron son "naughty" o "nice"
una string es "nice" si cumple:

 - Contiene al menos tres vocales (aeiou), como "aei", "xazegov" o "aeiouaeiouaeiou"
 - Contiene al menos una letra que aparece dos veces seguida, como "xx", "abcdde", "dd"
   "aabbccdd"
 - No contiene los strings: "ab", "cd", "pq" o "xy" aun si estas son parte de uno
   de los requerimientos anteriores(ver bien este que capaz que es el que nos esta
   jodiendo...)

## parte2

Ahora cambia el criterio de cuando una string es "nice" o "naughty" y ninguna
de las reglas anteriores aplican ahora. Ahora una string es "nice" si:

 - Contiene un par de letras que aparecen al menos dos veces en la string pero
   sin superponerse, como por ejemplo: "xyxy"(xy) o "aabcdefgaa" (aa), pero no
   como "aaa"(porque aa se superpone)
 - Contiene al menos una letra la cual alrededor de ella se repite otra, como
   por ejemplo "xyx", "abcdefeghi" (efe) o tambien "aaa"
