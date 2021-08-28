# Dia10

## Parte1

Hoy los elfos estan jugando a un juego llamado "mira y dice". El cual genera
secuencias de numeros leyendo la secuencia anterior y usando esa lectura para
generar la siguiente secuencia. Por ejemplo 211 se lee como "un dos, dos unos"
lo que lo convierte en 1221 (12, 21). Estas secuencias se generan iterativamente
usando el valor previo como entrada a el valor posterior. Para cada paso, tomar
el valor previo y reemplazar cada corrida de digitos (como 111) con el numero de
digitos que hay en ella (3 en este caso) seguido por el digito en si mismo (1)

por ejemplo:

 - 1 se convierte en 11 (1 copia del digito 1)
 - 11 se convierte en 21 (2 copias del digito 1)
 - 21 se convierte en 1211 (un 2 seguido por un 1)
 - 1211 se convierte en 111221 (un 1, un 2, y dos 1)
 - 111221 se convierte en 312211 (tres 1, dos 2, y un 1)

Comenzando con los digitos que nos dan como input, aplicar este proceso 40 veces
Cual es el largo del resultado???

## Parte 2

Ahora hay que aplicar el proceso 50 veces
