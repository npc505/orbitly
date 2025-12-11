# Orbitly: Sistema de Match por Gustos con Grafos (Rust + React + Neo4j)

Orbitly es una aplicación diseñada para conectar personas a partir de sus gustos (categorías, géneros e intereses específicos).

El sistema utiliza un grafo en Neo4j para modelar afinidades, permitiendo encontrar coincidencias de forma natural y eficiente.

## Modelo del Grafo

El grafo está compuesto por:

- **User** ’ persona que usa la aplicación
- **Interest** ’ un interés concreto (e.g., "Taylor Swift", "Star Wars", "Marvel Rivals")
- **Category** ’ categoría general (e.g., música, películas, actividades)
- **Genre** ’ subgénero o tipo específico dentro de una categoría

### Relaciones principales:

```cypher
(:User)-[:LIKES]->(:Interest)
(:Interest)-[:BELONGS_TO]->(:Category)
(:Interest)-[:HAS_GENRE]->(:Genre)
(:Interest)-[:HAS_SUBINTEREST]->(:Interest)
(:User)-[:MATCHES]->(:User)
```

---

## Arquitectura del Proyecto

Orbitly está dividido en tres capas tecnológicas:

### 1. Backend - Rust

Rust se eligió por:

- **Alto rendimiento** (compilado, sin garbage collector)
- **Seguridad de memoria** garantizada
- **Excelente manejo de concurrencia**
- Ideal para servicios que requieren responder queries de grafos rápidamente

**El backend expone:**

- Endpoints REST para autenticación y registro de usuarios
- Endpoints para crear intereses, categorías y géneros
- Endpoints para registrar relaciones (likes, matches)
- Lógica para calcular similitud entre usuarios usando cosine similarity
- Algoritmos de grafos avanzados (PageRank, Community Detection)

**Stack técnico:**

- **Axum**  framework web de alto rendimiento
- **neo4rs**  driver asíncrono para Neo4j
- **Tokio**  runtime asíncrono
- **Serde**  serialización/deserialización JSON

### 2. Base de Datos - Neo4j

Neo4j es el núcleo del sistema, permitiendo:

- **Modelar relaciones complejas** con nodos y aristas
- **Consultas naturalizadas de afinidad:**
  - "Usuarios que comparten más intereses"
  - "Intereses dentro de la misma categoría"
  - "Subintereses conectados"
- **Expansiones naturales** del modelo sin rediseño de tablas
- **Algoritmos de recomendación nativos** mediante GDS (Graph Data Science)

**Algoritmos implementados:**

- **Label Propagation**  detección de comunidades de usuarios
- **PageRank**  ranking de intereses por importancia
- **Node Similarity**  cálculo de similitud entre usuarios
- **Cosine Similarity**  scoring de compatibilidad basado en intereses compartidos
- **Shortest Path**  distancia entre usuarios en el grafo social

El diseño permite representar jerarquías de intereses y gustos con alto nivel de granularidad y flexibilidad.

### 3. Frontend - React + TypeScript

React + TypeScript fueron seleccionados porque:

- Son el **estándar moderno** para desarrollo web
- **TS permite tipado fuerte** y reduce errores en tiempo de ejecución
- React permite construir **componentes reutilizables**
- **Render rápido** mediante virtual DOM
- Facilita integración con librerías de visualización de grafos (e.g., react-force-graph, vis.js, d3-force)

**El frontend permite:**

- Crear perfil de usuario
- Seleccionar intereses
- Visualizar el grafo y tus conexiones
- Recibir recomendaciones de matches

---

## Decisiones de Modelado

###  Separar Interest, Category y Genre

Esto evita mezclar niveles semánticos:

- **Category** = nivel macro (ej. "Películas")
- **Genre** = clasificación interna (ej. "Ciencia ficción")
- **Interest** = elemento puntual (ej. "Star Wars")

###  Un usuario nunca conecta directo con Category o Genre

Siempre lo hace vía **Interest**, lo que:

- Mantiene **consistencia semántica**
- Facilita **recomendaciones basadas en nodos concretos**
- Evita conexiones irrelevantes ("likes Música" es muy amplio)

### Relaciones clave

- **LIKES** mide afinidad entre User e Interest
- **BELONGS_TO**  clasifica cada interés
- **HAS_GENRE**  crea filtrado temático
- **HAS_SUBINTEREST** permite encadenar intereses relacionados
- **MATCHES**  conecta usuarios con match mutuo

---

## API Endpoints

### Autenticación

#### Registrarse

```bash
POST /auth/signup
Content-Type: application/json

{
  "username": "oscar123",
  "mail": "oscar@example.com",
  "password": "SecurePass123",
  "first_name": "Oscar",
  "last_name": "Rodriguez",
  "description": "Amante de la ciencia ficción y los videojuegos",
  "avatar": "https://example.com/avatar.jpg"
}
```

#### Iniciar sesión

```bash
POST /auth/signin
Content-Type: application/json

{
  "username": "oscar123",
  "password": "SecurePass123"
}
```

**Respuesta:**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### Gestión de Intereses

#### Crear una categoría

```bash
POST /category
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Movies",
  "description": "Films and cinema"
}
```

#### Buscar categorías

```bash
POST /category/search
Authorization: Bearer <token>
Content-Type: application/json

{
  "term": "mov",
  "page": 0,
  "page_size": 20
}
```

#### Crear un género

```bash
POST /genre
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Sci-Fi",
  "description": "Science Fiction"
}
```

#### Buscar géneros

```bash
POST /genre/search
Authorization: Bearer <token>
Content-Type: application/json

{
  "term": "sci",
  "page": 0,
  "page_size": 20
}
```

### Intereses del Usuario

#### Ver mis intereses

```bash
GET /me/interest
Authorization: Bearer <token>
```

**Respuesta:**

```json
{
  "interests": [
    {
      "name": "Star Wars",
      "description": "Epic space opera franchise",
      "type": "Movie"
    },
    {
      "name": "Taylor Swift",
      "description": "American singer-songwriter",
      "type": "Music"
    }
  ]
}
```

#### Dar like a un interés

```bash
POST /me/interest
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Star Wars",
  "description": "Epic space opera franchise",
  "type": "Movie"
}
```

#### Quitar like a un interés

```bash
DELETE /me/interest
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Star Wars"
}
```

### Sistema de Matches

#### Hacer match con otro usuario

```bash
POST /me/match
Authorization: Bearer <token>
Content-Type: application/json

{
  "username": "maria_gamer"
}
```

#### Deshacer match

```bash
DELETE /me/match
Authorization: Bearer <token>
Content-Type: application/json

{
  "username": "maria_gamer"
}
```

#### Ver mis matches

```bash
GET /me/matches
Authorization: Bearer <token>
```

**Respuesta:**

```json
{
  "matches": [
    {
      "username": "maria_gamer",
      "first_name": "María",
      "last_name": "García",
      "description": "Gamer y fan del sci-fi",
      "avatar": "https://example.com/maria.jpg",
      "compatibility": 0.85
    }
  ]
}
```

#### Ver matches de nivel 2 (amigos de amigos)

```bash
GET /me/lv2
Authorization: Bearer <token>
```

### Recomendaciones

#### Obtener intereses recomendados

```bash
GET /me/recommendations
Authorization: Bearer <token>
```

**Respuesta:**

```json
{
  "recommendations": [
    {
      "name": "The Mandalorian",
      "description": "Star Wars series",
      "type": "TV Show",
      "score": 12.5,
      "source": "collaborative"
    },
    {
      "name": "Dune",
      "description": "Sci-fi epic",
      "type": "Movie",
      "score": 8.3,
      "source": "collaborative"
    }
  ]
}
```

### Búsqueda de Usuarios

#### Buscar usuarios (búsqueda relajada)

```bash
POST /other/search
Authorization: Bearer <token>
Content-Type: application/json

{
  "term": "mar",
  "page": 0,
  "page_size": 10
}
```

**Respuesta:**

```json
{
  "matches": [
    {
      "username": "maria_gamer",
      "first_name": "María",
      "last_name": "García",
      "description": "Gamer y fan del sci-fi",
      "avatar": "https://example.com/maria.jpg",
      "compatibility": 0.72
    }
  ]
}
```

#### Buscar usuarios (búsqueda estricta con fuzzy matching)

```bash
POST /other/search/strict
Authorization: Bearer <token>
Content-Type: application/json

{
  "term": "maria",
  "page": 0,
  "page_size": 10
}
```

### Información de Otros Usuarios

#### Ver perfil de otro usuario

```bash
POST /other
Authorization: Bearer <token>
Content-Type: application/json

{
  "username": "maria_gamer"
}
```

**Respuesta:**

```json
{
  "username": "maria_gamer",
  "first_name": "María",
  "last_name": "García",
  "description": "Gamer y fan del sci-fi",
  "avatar": "https://example.com/maria.jpg",
  "compatibility": 0.85
}
```

#### Ver matches de otro usuario

```bash
POST /other/matches
Authorization: Bearer <token>
Content-Type: application/json

{
  "username": "maria_gamer"
}
```

#### Ver intereses de otro usuario

```bash
POST /other/interest
Authorization: Bearer <token>
Content-Type: application/json

{
  "username": "maria_gamer"
}
```

### Análisis de Grafos

#### Obtener comunidades de usuarios

```bash
GET /comunidades
Authorization: Bearer <token>
```

**Respuesta:**

```json
{
  "communities": [
    {
      "username": "oscar123",
      "community_id": 1
    },
    {
      "username": "maria_gamer",
      "community_id": 1
    },
    {
      "username": "juan_dev",
      "community_id": 2
    }
  ]
}
```

#### Obtener ranking de intereses (PageRank)

```bash
GET /pagerank
Authorization: Bearer <token>
```

**Respuesta:**

```json
{
  "rankings": [
    {
      "name": "Star Wars",
      "score": 0.245
    },
    {
      "name": "Marvel",
      "score": 0.198
    },
    {
      "name": "Taylor Swift",
      "score": 0.156
    }
  ]
}
```

#### Encontrar camino más corto entre usuarios

```bash
POST /me/shortest-path
Authorization: Bearer <token>
Content-Type: application/json

{
  "username": "juan_dev"
}
```

**Respuesta:**

```json
{
  "path_length": 2,
  "nodes": ["oscar123", "maria_gamer", "juan_dev"]
}
```

### Información del Usuario Actual

#### Ver mi perfil

```bash
GET /me
Authorization: Bearer <token>
```

**Respuesta:**

```json
{
  "username": "oscar123",
  "first_name": "Oscar",
  "last_name": "Rodriguez",
  "description": "Amante de la ciencia ficción y los videojuegos",
  "avatar": "https://example.com/avatar.jpg",
  "compatibility": 1.0
}
```

---

## Ejemplos de Queries Cypher

### Obtener todos los intereses de un usuario

```cypher
MATCH (u:User{username: $username})-[:LIKES]->(i:Interest)
RETURN i.name AS name, i.description AS description, i.type AS type
```

### Encontrar usuarios con intereses similares

```cypher
MATCH (u1:User{username: $username})-[:LIKES]->(i1:Interest),
      (u2:User)-[:LIKES]->(i2:Interest)
WITH COLLECT(ID(i1)) AS u1_likes, COLLECT(ID(i2)) AS u2_likes, u1, u2
WITH u1, u2, gds.similarity.cosine(u1_likes, u2_likes) AS compatibility
WHERE u1 <> u2
RETURN u2.username, compatibility
ORDER BY compatibility DESC
```

### Recomendaciones colaborativas

```cypher
MATCH (u:User{username: $username})-[:LIKES]->(:Interest)<-[:LIKES]-(a:User)-[:LIKES]->(i:Interest)
WHERE NOT (u)-[:LIKES]->(i)
WITH DISTINCT a, i
RETURN i.name AS name, i.description AS description, i.type AS type, toFloat(COUNT(a)) AS score
ORDER BY score DESC
LIMIT 30
```

### Detección de comunidades con Label Propagation

```cypher
MATCH (source:User)-[r:MATCHES]->(target:User)
WITH gds.graph.project('matchGraph', source, target) AS g
RETURN g.graphName AS graph

CALL gds.labelPropagation.stream('matchGraph')
YIELD nodeId, communityId
RETURN gds.util.asNode(nodeId).username AS username, communityId
ORDER BY communityId, username

CALL gds.graph.drop('matchGraph', false)
```

### PageRank de intereses

```cypher
CALL gds.graph.project(
    'pageRankGraph',
    'Interest',
    '*'
)

CALL gds.pageRank.stream('pageRankGraph')
YIELD nodeId, score
RETURN gds.util.asNode(nodeId).name AS name, score
ORDER BY score DESC

CALL gds.graph.drop('pageRankGraph', false)
```

---

## Configuración y Ejecución

### Requisitos

- **Rust** 1.70+
- **Neo4j** 5.0+ con plugin GDS
- **Node.js** 18+ (para frontend)

### Backend (Rust)

```bash
cd backend
cargo build --release
cargo run
```

El servidor escuchará en `http://localhost:8000`

### Base de Datos (Neo4j)

```bash
# Iniciar Neo4j
neo4j start

# Verificar que GDS esté instalado
CALL gds.list()
```

### Frontend (React)

```bash
cd frontend
npm install
npm run dev
```

El frontend estará disponible en `http://localhost:5173`

---

## Variables de Entorno

Crear archivo `.env` en el directorio `backend/`:

```env
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=your_password
JWT_SECRET=your_jwt_secret_key
```

---

## Características Principales

 **Autenticación segura** con JWT y bcrypt
 **Scoring de compatibilidad** usando cosine similarity
 **Recomendaciones colaborativas** basadas en usuarios similares
 **Detección de comunidades** con Label Propagation
 **Ranking de intereses** con PageRank
 **Búsqueda fuzzy** de usuarios con múltiples estrategias
 **Paginación** en todas las búsquedas
 **Caminos más cortos** entre usuarios en el grafo social
 **Sistema de matches bidireccional**

---

## Contribuciones

Este proyecto es parte de un trabajo universitario de redes y grafos.

**Equipo:**
- Backend & Database: [Tu nombre]
- Frontend: [Nombre del compañero de frontend]

---

## Licencia

Este proyecto es de código abierto bajo la licencia MIT.
