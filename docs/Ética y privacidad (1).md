Política de uso de datos

*<u>Fuente de datos</u>*

En este proyecto se emplearon tres tipos principales de datos: **1.**
**Datasets** **públicos** **de** **Kaggle**

> \- Spotify Global Music Dataset (2009-2025) - The Movie Dataset
> (movies_metadata.csv)

Kaggle permite distribuir estos datos bajo licencia abierta para
utilizarlos en análisis, investigaciones y desarrollo de modelos.

> **2.** **Datos** **sintéticos** **generados** **con** **la**
> **librería** **Faker**

Se crearon perfiles de usuarios (con nombres, usuarios, correos,
avatares, descripciones) solo con fines para simular las cuentas en la
aplicación. Estos datos no representan personas reales.

> **3.** **Datos** **generados** **con** **ChatGPT**

Se utilizaron respuestas dadas por IA para enriquecer el dataset con
usuarios, intereses, categorías y géneros. Este contenido no proviene de
personas reales, todo es sintético.

*<u>Posibles sesgos en los datos</u>*

Los datos pueden tener los siguientes sesgos debido a sus orígenes:

> \- Los datos de Kaggle pueden tener sesgos por una
> sobrerrepresentación de artistas y películas con mayor presencia en la
> industria.
>
> \- Los datos sintéticos con librerías e IA pueden tener distribuciones
> que no reflejan poblaciones reales o diversas.

*<u>Medidas de mitigación de sesgos</u>* Para reducir riesgos

> \- Normalizamos y limpiamos datos para evitar datos duplicados y para
> validarlos.
>
> \- Se podría balancear los datasets para tener una mejor distribución
> entre películas y artistas.
>
> \- Generar datos específicos de regiones para tener una simulación más
> real.

*<u>Privacidad y anonimización</u>*

Aunque el proyecto no utiliza datos de personas reales, se siguen los
siguientes principios:

> \- **Anonimización**: no se almacenan datos reales que permitan
> identificar a una persona y los identificadores son sintéticos.
>
> \- **No** **generar** **datos** **sensibles** **reales**: todos los
> intereses y datos personales son generados por IA o librerías.
>
> \- **Uso** **solo** **para** **demostración** **técnica**: Los datos
> solo se emplean para enseñar el funcionamiento, no para usarlos en la
> vida real.
