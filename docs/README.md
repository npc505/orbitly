# Orbitly: Sistema de Match por Gustos con Grafos (Rust + React + Neo4j)

Orbitly es una aplicación diseñada para conectar personas a partir de sus gustos (categorías, géneros e intereses específicos).

El sistema utiliza un grafo en Neo4j para modelar afinidades, permitiendo encontrar coincidencias de forma natural y eficiente.

## Modelo del Grafo

El grafo está compuesto por:

- '**User**' persona que usa la aplicación
- '**Interest**' un interés concreto (e.g., "Taylor Swift", "Star Wars", "Marvel Rivals")
- '**Category**' categoría general (e.g., música, películas, actividades)
- '**Genre**' subgénero o tipo específico dentro de una categoría

### Relaciones principales:

```cypher
(:User)-[:LIKES]->(:Interest)
(:Interest)-[:BELONGS_TO]->(:Category)
(:Interest)-[:HAS_GENRE]->(:Genre)
(:Interest)-[:HAS_SUBINTEREST]->(:Interest)
(:User)-[:MATCHES]->(:User)
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

**Autenticación segura** con JWT y bcrypt
**Scoring de compatibilidad** usando cosine similarity
**Recomendaciones colaborativas** basadas en usuarios similares
**Detección de comunidades** con Label Propagation
**Ranking de intereses** con PageRank
**Búsqueda fuzzy** de usuarios con múltiples estrategias
**Paginación** en todas las búsquedas
**Caminos más cortos** entre usuarios en el grafo social
**Sistema de matches bidireccional**

---

## Contribuciones

Este proyecto es parte de un trabajo universitario de redes y grafos.


---

## Licencia

Este proyecto es de código abierto bajo la licencia MIT.
