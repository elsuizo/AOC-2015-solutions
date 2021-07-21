# Dia4

## Parte1
Santa necesita ayuda minando algunos AdventCoins (muy similares a los bitcoins)
Para hacer esto necesita encontrar hashes MD5 los cuales en hexa comienzan con
al menos 5 ceros. La entrada a el MD5 es una llave secreta (el input del problema)
seguido de un numero en decimal. Para minar AdventCoins debemos primero el numero
positivo mas pequenio (sin ceros a la izquierda) que producen dicho hash

Por ejemplo:

si la palabra secreta es abcdef la respuesta debe ser: 609043 porque el hash MD5
de "abcdef609043" comienza con cinco ceros: 000001dbbfa... y es el menor numero
que puede hacer eso

Si la palabra secreta es pqrstuv el menor numero que combinado con un hash MD5
puede hacer eso es 1048970 que es el hash de pqrstuv1048970 sea: 000006136ef...

## Parte2

ahora tiene que comenzar con 6 ceros
