# **demo – Orbitly (Markdown)**

## **MELANY**

Hola, mi nombre es Melany y hoy les voy a presentar la aplicación en la que estuvimos trabajando mi equipo y yo a lo largo de todo este trimestre para la clase de minería de datos.

Este es nuestro login. Nuestra aplicación se llama **Orbitly** porque hicimos referencia a las órbitas de los planetas: sentimos que las personas se pueden conectar de tal manera haciendo *matching*, como si las personas que están en tu órbita fueran a conectar contigo. Esa es nuestra analogía del por qué elegimos este nombre.

Entonces tenemos aquí nuestro login; yo aquí tengo un usuario, entonces voy a entrar.


¿Y qué es lo que vamos a encontrar? Bien, esto es nuestro inicio. Como podemos ver, aquí están nuestros *matches*, que es la gente con la que he podido conectar debido a mis intereses.

Esto responde técnicamente una de las preguntas: ¿por qué esta aplicación? ¿Qué tiene de diferente en comparación con otras aplicaciones o redes sociales que ya existen?

Pues que esta no solo te permite entablar una relación amorosa, sino que también te permite construir amistades, lazos afectivos y distintos tipos de relaciones.
Lo interesante aquí es *cómo* se hace el match: conforme a tus intereses, tu banda favorita, tu cantante favorito, tu canción favorita… Yo puedo tener una canción que escucho todo el día 
y decir: “Wow, esta persona también está obsesionada con esta canción”; entonces automáticamente voy a querer ser su amiga, ¿vale?

Esto es lo que hace esta aplicación: compartir intereses. Y todo es *match automático*, eso es lo interesante.

Aquí tenemos nuestros *matches*, que son todas las personas con las que he conectado, mis intereses, los chats recientes, personas sugeridas y lo que está en tendencia. Vamos uno por uno.

### **Matches**

Como mencioné, están todas las personas con las que he conectado, y aquí podemos verlas y la compatibilidad que tengo con ellas. Por ejemplo, quiero ver el perfil de una de estas personas.

Aquí podemos ver su perfil. Tenemos el icono de dejar de seguir y los intereses. ¿Por qué tenemos dejar de seguir? Porque ya tenemos conexión con esta persona.

Aquí tenemos sus matches y sus intereses, que son lo que nos importa. Si a mí también me gusta esa canción, puedo entablar conversación con esa persona. Si quisiera dejar de seguir, se puede. Vamos a dejar de seguir a ver qué pasa.

Y pues ahí está: se dejó de seguir, ya no sale. Como podemos ver, desapareció esta persona de aquí.

### **Intereses**

Aquí tenemos nuestros intereses, la segunda sección. Podemos ver todos los intereses que tenemos hoy en día, y como sabemos, cambian mucho.

Tardó un poco en cargar porque son más de 150 intereses. Si quiero agregar más y no me gusta ninguno, le doy 
en “buscar más intereses” para refrescar y obtener más opciones. Pero a mí sí me gusta *Rosé*, así que voy a agregarlo. Al hacerlo, me aparece inmediatamente.

Si ya no me interesa *Elizabeth Taylor de Taylor Swift*, la puedo quitar. Confirmo y desaparece.

Si volvemos a nuestro feed, ya aparece *Rosé* aquí y *Elizabeth Taylor* ya no está.

### **Chats**

Ahora va la parte de los chats. Aquí yo hice algunas pruebas, por eso aparecen así (“oye por qué no me contestas”, “hola”). Todo queda registrado.

Esto es algo que queremos optimizar en el futuro.

### **Personas sugeridas**

Aquí sale la compatibilidad y la imagen de perfil. Puedo ver el perfil, ver intereses y matches (si ya conecté). Si quiero conectar, le doy en “conectar”.

Regreso al inicio y ya está en mis matches.

### **Tendencias**

En tendencias vemos qué gustos están en el top. Aquí aparecen bastantes.

Este es nuestro prototipo hasta ahora. Podemos agregar muchos gustos, por ejemplo *Bye Bye Love*, y se agregan automáticamente. También se refrescan opciones.

Podemos crear usuarios y registrarlos sin problema. Guardan la información y podemos iniciar sesión.

Creo que eso es todo. Espero que les guste nuestra aplicación.
Gracias por todo.

---

## **Hablante B**

Nuestra aplicación conecta personas a partir de sus gustos. Modelamos usuarios, intereses, categorías y géneros en un grafo de Neo4j. Esto nos permite ver cómo se relacionan, 
detectar patrones y ver afinidades entre usuarios.

Hicimos varias consultas para que funcionara todo.

La primera consulta analiza cuáles categorías fueron más populares en el último mes. Busca los *likes* más recientes de 30 días y
relaciona usuarios con intereses y con categorías para agruparlas según su actividad. Esto nos muestra el top de categorías del mes y qué usuarios contribuyeron.

La ejecutamos y vemos que **Música** y **Sport** fueron las más populares, gracias a *Carlos27* y *Ana25*.

La segunda consulta calcula qué tan parecidos son los usuarios usando **similitud coseno**.
Cada usuario se convierte en un vector de intereses y devuelve un número entre 0 y 1, donde 1 es gustos idénticos y 0 es nulo. La ejecutamos y vemos la similitud entre usuarios.

Además de consultas directas, hicimos un **pipeline de grafos con GDS**.
Primero proyectamos un subgrafo dentro de GDS que nos une usuarios con géneros. Luego ejecutamos el algoritmo **PageRank**, que nos permite identificar qué géneros son más influyentes dentro de la red de gustos: qué gustos dominan y conectan a más usuarios.

En resumen, la aplicación usa grafos para entender preferencias reales, consultar actividades, similitudes y afinidades, y eso convierte esta aplicación en un sistema de **recomendaciones inteligentes**.


