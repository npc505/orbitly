Orbitly — Sistema de Match por Gustos con Grafos (Rust + React + Neo4j)

Orbitly es una aplicación diseñada para conectar personas a partir de sus gustos (categorías, géneros e intereses específicos).

El sistema utiliza un grafo en Neo4j para modelar afinidades, permitiendo encontrar coincidencias de forma natural y eficiente.

El grafo está compuesto por:
User → persona que usa la aplicación
Interest → un interés concreto (e.g., “Taylor Swift”, “Star Wars”, “Marvel Rivals")
Category → categoría general (e.g., música, películas, actividades)
Genre → subgénero o tipo específico dentro de una categoría
Relaciones principales:
(:User)-[:LIKES]->(:Interest)
(:Interest)-[:BELONGS_TO]->(:Category)
(:Interest)-[:HAS_GENRE]->(:Genre)
(:Interest)-[:HAS_SUBINTEREST]->(:Interest)
(:User)-[:MATCHES]->(:User)


Arquitectura del Proyecto
Orbitly está dividido en tres capas tecnológicas:
1. Backend — Rust
    Rust se eligió por:
    Alto rendimiento (compilado, sin garbage collector)
    Seguridad de memoria garantizada
    Excelente manejo de concurrencia
    Ideal para servicios que requieren responder queries de grafos rápidamente
    El backend expone:
    Endpoints REST para registrar usuarios
    Endpoints para crear intereses, categorías y géneros
    Endpoints para registrar relaciones
    Lógica para calcular similitud entre usuarios usando Neo4j



2. Base de Datos — Neo4j
    Neo4j es el núcleo del sistema, permitiendo:
        Modelar relaciones complejas con nodos y aristas
        Consultas naturalizadas de afinidad:
        “Usuarios que comparten más intereses”
        “Intereses dentro de la misma categoría”
        “Subintereses conectados”
        Expansiones naturales del modelo sin rediseño de tablas
        Algoritmos de recomendación nativos mediante GDS (Graph Data Science)
        El diseño permite representar jerarquías de intereses y gustos con alto nivel de granularidad y flexibilidad.
        
3. Frontend — React + TypeScript
    React + TypeScript fueron seleccionados porque:
    Son el estándar moderno para desarrollo web
    TS permite tipado fuerte y reduce errores en tiempo de ejecución
    React permite construir componentes reutilizables
    Render rápido mediante virtual DOM
    Facilita integración con librerías de visualización de grafos (e.g., react-force-graph, vis.js, d3-force)
    El frontend permite:
    Crear perfil de usuario
    Seleccionar intereses
    Visualizar el grafo y tus conexiones
    Recibir recomendaciones de maches
        
    
Decisiones de Modelado
    * Separar Interest, Category y Genre
        Esto evita mezclar niveles semánticos:
        Category = nivel macro (ej. “Películas”)
        Genre = clasificación interna (ej. “Ciencia ficción”)
        Interest = elemento puntual (ej. “Star Wars”)
    * Un usuario nunca conecta directo con Category o Genre
        Siempre lo hace vía Interest, lo que:
            *Mantiene consistencia semántica
            *Facilita recomendaciones basadas en nodos concretos
            *Evita conexiones irrelevantes (“likes Música” es muy amplio)
    * Relaciones clave
        LIKES mide afinidad entre User e Interest
        BELONGS_TO clasifica cada interés
        HAS_GENRE crea filtrado temático
        HAS_SUBINTEREST permite encadenar intereses relacionados
        

Ejemplos de Uso (API)
* Crear un usuario
    POST /users
    {
      "id": "u123",
      "name": "Oscar"
    }
* Registrar un interés
    POST /interests
    {
      "name": "Star Wars",
      "category": "Movies",
      "genre": "Sci-Fi"
    }
    El backend creará:
    un nodo Interest
    un nodo Category si no existe
    un nodo Genre si no existe
    sus relaciones

* El usuario indica un interés
    POST /users/u123/likes
    {
      "interest": "Star Wars"
    }

* Obtener usuarios similares
    GET /users/u123/matches
    Respuesta:
    {
      "user": "u123",
      "matches": [
        { "id": "u98"},
        { "id": "u201"}
      ]
    }
    
    
* Ejemplo Cypher — Intereses compartidos
    MATCH (:User{username: $username})-[:LIKES]->(i:Interest) RETURN i
