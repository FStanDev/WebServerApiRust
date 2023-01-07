# WebServerApiRust

Este es el repositorio de código del tutorial de Rust para crear un webserver con actix, conectarnos a una BBDD Postgresql con Diesel y utilizar templates html con Tera, todo en Rust.

Vídeo Tutorial [aquí:](https://youtu.be/bqociCzcXX4) 

## Configuración previa

Antes de ejecutar el código, tienes que tener Postgresql instalado en tu ordenador, junto a las herramientas de desarrollo de Postgresql para poder conectarte a la BBDD desde Rust.

También tienes que cambiar, dentro del archivo .env, el usuario y la contraseña, por defecto están a TuUsuario y TuContraseña:

`DATABASE_URL=postgres://TuUsuario:TuContraseña@localhost/GarageTeam`

Debes crear una BBDD en Postgresql que se llame GarageTeam y ejecutar los scripts de creación de las tablas, el script completo de creación, incluyendo la BBDD es este:

```
--
-- PostgreSQL database dump
--

-- Dumped from database version 14.5 (Ubuntu 14.5-0ubuntu0.22.04.1)
-- Dumped by pg_dump version 14.5 (Ubuntu 14.5-0ubuntu0.22.04.1)

-- Started on 2023-01-06 18:23:47 CET

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 211 (class 1259 OID 16395)
-- Name: CarToRepair; Type: TABLE; Schema: public; Owner: stantech
--

CREATE TABLE public."CarToRepair" (
    id uuid NOT NULL,
    modelo character(50) NOT NULL,
    fecha_entrada timestamp with time zone NOT NULL,
    fecha_salida timestamp with time zone NOT NULL
);


ALTER TABLE public."CarToRepair" OWNER TO stantech;

--
-- TOC entry 210 (class 1259 OID 16389)
-- Name: Garage; Type: TABLE; Schema: public; Owner: stantech
--

CREATE TABLE public."Garage" (
    id bigint NOT NULL,
    name character(50) NOT NULL,
    location character(150) NOT NULL,
    capacity integer NOT NULL
);


ALTER TABLE public."Garage" OWNER TO stantech;

--
-- TOC entry 209 (class 1259 OID 16388)
-- Name: Garage_id_seq; Type: SEQUENCE; Schema: public; Owner: stantech
--

CREATE SEQUENCE public."Garage_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Garage_id_seq" OWNER TO stantech;

--
-- TOC entry 3386 (class 0 OID 0)
-- Dependencies: 209
-- Name: Garage_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: stantech
--

ALTER SEQUENCE public."Garage_id_seq" OWNED BY public."Garage".id;


--
-- TOC entry 214 (class 1259 OID 16402)
-- Name: WorkingCars; Type: TABLE; Schema: public; Owner: stantech
--

CREATE TABLE public."WorkingCars" (
    id bigint NOT NULL,
    assigned_garage bigint NOT NULL,
    car_to_repair uuid NOT NULL
);


ALTER TABLE public."WorkingCars" OWNER TO stantech;

--
-- TOC entry 213 (class 1259 OID 16401)
-- Name: WorkingCars_assigned_garage_seq; Type: SEQUENCE; Schema: public; Owner: stantech
--

CREATE SEQUENCE public."WorkingCars_assigned_garage_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."WorkingCars_assigned_garage_seq" OWNER TO stantech;

--
-- TOC entry 3387 (class 0 OID 0)
-- Dependencies: 213
-- Name: WorkingCars_assigned_garage_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: stantech
--

ALTER SEQUENCE public."WorkingCars_assigned_garage_seq" OWNED BY public."WorkingCars".assigned_garage;


--
-- TOC entry 212 (class 1259 OID 16400)
-- Name: WorkingCars_id_seq; Type: SEQUENCE; Schema: public; Owner: stantech
--

CREATE SEQUENCE public."WorkingCars_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."WorkingCars_id_seq" OWNER TO stantech;

--
-- TOC entry 3388 (class 0 OID 0)
-- Dependencies: 212
-- Name: WorkingCars_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: stantech
--

ALTER SEQUENCE public."WorkingCars_id_seq" OWNED BY public."WorkingCars".id;


--
-- TOC entry 215 (class 1259 OID 16469)
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: stantech
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO stantech;

--
-- TOC entry 3221 (class 2604 OID 16392)
-- Name: Garage id; Type: DEFAULT; Schema: public; Owner: stantech
--

ALTER TABLE ONLY public."Garage" ALTER COLUMN id SET DEFAULT nextval('public."Garage_id_seq"'::regclass);


--
-- TOC entry 3222 (class 2604 OID 16405)
-- Name: WorkingCars id; Type: DEFAULT; Schema: public; Owner: stantech
--

ALTER TABLE ONLY public."WorkingCars" ALTER COLUMN id SET DEFAULT nextval('public."WorkingCars_id_seq"'::regclass);


--
-- TOC entry 3223 (class 2604 OID 16406)
-- Name: WorkingCars assigned_garage; Type: DEFAULT; Schema: public; Owner: stantech
--

ALTER TABLE ONLY public."WorkingCars" ALTER COLUMN assigned_garage SET DEFAULT nextval('public."WorkingCars_assigned_garage_seq"'::regclass);


--
-- TOC entry 3376 (class 0 OID 16395)
-- Dependencies: 211
-- Data for Name: CarToRepair; Type: TABLE DATA; Schema: public; Owner: stantech
--

COPY public."CarToRepair" (id, modelo, fecha_entrada, fecha_salida) FROM stdin;
\.


--
-- TOC entry 3375 (class 0 OID 16389)
-- Dependencies: 210
-- Data for Name: Garage; Type: TABLE DATA; Schema: public; Owner: stantech
--

COPY public."Garage" (id, name, location, capacity) FROM stdin;
\.


--
-- TOC entry 3379 (class 0 OID 16402)
-- Dependencies: 214
-- Data for Name: WorkingCars; Type: TABLE DATA; Schema: public; Owner: stantech
--

COPY public."WorkingCars" (id, assigned_garage, car_to_repair) FROM stdin;
\.


--
-- TOC entry 3380 (class 0 OID 16469)
-- Dependencies: 215
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: stantech
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
20230106154656	2023-01-06 15:52:26.803027
\.


--
-- TOC entry 3389 (class 0 OID 0)
-- Dependencies: 209
-- Name: Garage_id_seq; Type: SEQUENCE SET; Schema: public; Owner: stantech
--

SELECT pg_catalog.setval('public."Garage_id_seq"', 2, true);


--
-- TOC entry 3390 (class 0 OID 0)
-- Dependencies: 213
-- Name: WorkingCars_assigned_garage_seq; Type: SEQUENCE SET; Schema: public; Owner: stantech
--

SELECT pg_catalog.setval('public."WorkingCars_assigned_garage_seq"', 1, false);


--
-- TOC entry 3391 (class 0 OID 0)
-- Dependencies: 212
-- Name: WorkingCars_id_seq; Type: SEQUENCE SET; Schema: public; Owner: stantech
--

SELECT pg_catalog.setval('public."WorkingCars_id_seq"', 11, true);


--
-- TOC entry 3228 (class 2606 OID 16399)
-- Name: CarToRepair CarToRepair_pkey; Type: CONSTRAINT; Schema: public; Owner: stantech
--

ALTER TABLE ONLY public."CarToRepair"
    ADD CONSTRAINT "CarToRepair_pkey" PRIMARY KEY (id);


--
-- TOC entry 3226 (class 2606 OID 16394)
-- Name: Garage Garage_pkey; Type: CONSTRAINT; Schema: public; Owner: stantech
--

ALTER TABLE ONLY public."Garage"
    ADD CONSTRAINT "Garage_pkey" PRIMARY KEY (id);


--
-- TOC entry 3230 (class 2606 OID 16408)
-- Name: WorkingCars WorkingCars_pkey; Type: CONSTRAINT; Schema: public; Owner: stantech
--

ALTER TABLE ONLY public."WorkingCars"
    ADD CONSTRAINT "WorkingCars_pkey" PRIMARY KEY (id);


--
-- TOC entry 3232 (class 2606 OID 16474)
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: stantech
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- TOC entry 3234 (class 2606 OID 16414)
-- Name: WorkingCars CocheAsignado; Type: FK CONSTRAINT; Schema: public; Owner: stantech
--

ALTER TABLE ONLY public."WorkingCars"
    ADD CONSTRAINT "CocheAsignado" FOREIGN KEY (car_to_repair) REFERENCES public."CarToRepair"(id);


--
-- TOC entry 3233 (class 2606 OID 16409)
-- Name: WorkingCars GarajeAsignado; Type: FK CONSTRAINT; Schema: public; Owner: stantech
--

ALTER TABLE ONLY public."WorkingCars"
    ADD CONSTRAINT "GarajeAsignado" FOREIGN KEY (assigned_garage) REFERENCES public."Garage"(id);


-- Completed on 2023-01-06 18:23:47 CET

--
-- PostgreSQL database dump complete
--
```
Con esto ya configurado, no deberías tener problemas.

## Tareas

Si quieres, puedes lanzar tu pull request con los siguienets cambios que he pedido en el vídeo:

1. Que se tenga en cuenta la capacidad de los garajes
2. Que si un coche ya está asignado, no se pueda volver a asignar
3. Poder elegir a que garaje asignar

Puedes hacer una pull request con tus cambios y revisaré, y si es correcto, lo integraré en el repo 


