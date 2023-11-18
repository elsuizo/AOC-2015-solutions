# Dia11: Politica de corporacion

El password previo de santa ha expirado y el necesita elegir uno nuevo. Para
ayudarlo a que se acuerde de el nuevo password santa ha hecho un metodo para
generar el nuevo password basandose en el anterior. Las politicas de corporacion
dicen que los passwords tienen que tener exactamente 8 letras minusculas(por
razones de seguridad) entonces el encuentra su nuevo password incrementando su
password viejo repetidamente hasta que sea uno valido(es por esto que quiero
hacerlo con iteradores...)
Incrementar es como contar con numeros: `xx`, `xy`, `xz`, `ya`, `yb` y asi
sucesivamente. Incrementar la letra que esta mas a la derecha una vez; si esta
fue una `z` tiene que hacer el wrap para comenzar de nuevo en la `a` y repetir
con la proxima letra a la izquierda hasta que no haya mas wraps

Desafortunadamente para santa un nuevo elfo de seguridad recientemente ha
comenzado con su trabajo y impuso algunas reglas adicionales a los
requerimientos

 - Los passwords deben incluir una serie de tres letras consecutivas como por
   ejemplo: `abc`, `bcd`, `cde` y asi hasta `xyz`. No puede saltearse letras por
   ejemplo: `abd` no cuenta
 - Los passwords deben no contener las letras `i`, `o` o `l` ya que estas letras
   se pueden confundir muy facilmente por otros caracteres
 - Los passwords deben contener al menos dos diferentes pares de letras que se
   repitan como por ejemplo: `aa`, `bb` o `zz`

Por ejemplo:

 - `hijklmmn`: cumple el primer requerimiento (porque contiene las tres letras
   consecutivas) pero falla en el segundo requerimiento (porque contiene las
   letras prohibidas `i` y `l`)
 - `abbceffg`: cumple con el tercer requerimiento (porque se repiten las `bb` y
   `ff` pero falla en el primer requerimiento)
 - `abbcegjk`: falla en el tercer requerimiento porque solo tienen una doble
   letra repetida (`bb`)
 - el proximo password a `abbcegjk` es `abcdffaa`
 - el proximo password despues de `ghijklmn` es `ghjaabcc` porque vamos a
   saltear todos los passwords que comiencen con `ghi...` dado que `i` no esta
   perimitido

Dado el password actual de santa (nuestro input) cual deberia ser su proximo
password???

Nuestra input es: `vzbxkghb`
