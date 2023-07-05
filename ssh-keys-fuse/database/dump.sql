-- create_table.sql

CREATE TABLE IF NOT EXISTS ssh_keys (
    user_id TEXT,
    value TEXT
);

-- insert_data.sql

INSERT INTO ssh_keys (user_id, value)
VALUES ('user1', 'key1'),
       ('user2', 'key2'),
       ('user3', 'key3');


-- --
-- -- PostgreSQL database dump
-- --

-- -- Dumped from database version 15.3 (Debian 15.3-1.pgdg120+1)
-- -- Dumped by pg_dump version 15.3 (Debian 15.3-1.pgdg120+1)

-- SET statement_timeout = 0;
-- SET lock_timeout = 0;
-- SET idle_in_transaction_session_timeout = 0;
-- SET client_encoding = 'UTF8';
-- SET standard_conforming_strings = on;
-- SELECT pg_catalog.set_config('search_path', '', false);
-- SET check_function_bodies = false;
-- SET xmloption = content;
-- SET client_min_messages = warning;
-- SET row_security = off;

-- SET default_tablespace = '';

-- SET default_table_access_method = heap;

-- --
-- -- Name: _prisma_migrations; Type: TABLE; Schema: public; Owner: api
-- --

-- CREATE TABLE public._prisma_migrations (
--     id character varying(36) NOT NULL,
--     checksum character varying(64) NOT NULL,
--     finished_at timestamp with time zone,
--     migration_name character varying(255) NOT NULL,
--     logs text,
--     rolled_back_at timestamp with time zone,
--     started_at timestamp with time zone DEFAULT now() NOT NULL,
--     applied_steps_count integer DEFAULT 0 NOT NULL
-- );


-- ALTER TABLE public._prisma_migrations OWNER TO api;

-- --
-- -- Name: deployments; Type: TABLE; Schema: public; Owner: api
-- --

-- CREATE TABLE public.deployments (
--     id integer NOT NULL,
--     uuid character(32) NOT NULL,
--     name character varying(40) NOT NULL,
--     config jsonb NOT NULL,
--     created_at timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
--     updated_at timestamp(3) without time zone NOT NULL,
--     project_id integer NOT NULL
-- );


-- ALTER TABLE public.deployments OWNER TO api;

-- --
-- -- Name: deployments_id_seq; Type: SEQUENCE; Schema: public; Owner: api
-- --

-- CREATE SEQUENCE public.deployments_id_seq
--     AS integer
--     START WITH 1
--     INCREMENT BY 1
--     NO MINVALUE
--     NO MAXVALUE
--     CACHE 1;


-- ALTER TABLE public.deployments_id_seq OWNER TO api;

-- --
-- -- Name: deployments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: api
-- --

-- ALTER SEQUENCE public.deployments_id_seq OWNED BY public.deployments.id;


-- --
-- -- Name: projects; Type: TABLE; Schema: public; Owner: api
-- --

-- CREATE TABLE public.projects (
--     id integer NOT NULL,
--     uuid character(32) NOT NULL,
--     name character varying(40) NOT NULL,
--     created_at timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
--     updated_at timestamp(3) without time zone NOT NULL,
--     user_id integer NOT NULL
-- );


-- ALTER TABLE public.projects OWNER TO api;

-- --
-- -- Name: projects_id_seq; Type: SEQUENCE; Schema: public; Owner: api
-- --

-- CREATE SEQUENCE public.projects_id_seq
--     AS integer
--     START WITH 1
--     INCREMENT BY 1
--     NO MINVALUE
--     NO MAXVALUE
--     CACHE 1;


-- ALTER TABLE public.projects_id_seq OWNER TO api;

-- --
-- -- Name: projects_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: api
-- --

-- ALTER SEQUENCE public.projects_id_seq OWNED BY public.projects.id;


-- --
-- -- Name: ssh_keys; Type: TABLE; Schema: public; Owner: api
-- --

-- CREATE TABLE public.ssh_keys (
--     id integer NOT NULL,
--     value text NOT NULL,
--     created_at timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
--     updated_at timestamp(3) without time zone NOT NULL,
--     user_id integer NOT NULL
-- );


-- ALTER TABLE public.ssh_keys OWNER TO api;

-- --
-- -- Name: ssh_keys_id_seq; Type: SEQUENCE; Schema: public; Owner: api
-- --

-- CREATE SEQUENCE public.ssh_keys_id_seq
--     AS integer
--     START WITH 1
--     INCREMENT BY 1
--     NO MINVALUE
--     NO MAXVALUE
--     CACHE 1;


-- ALTER TABLE public.ssh_keys_id_seq OWNER TO api;

-- --
-- -- Name: ssh_keys_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: api
-- --

-- ALTER SEQUENCE public.ssh_keys_id_seq OWNED BY public.ssh_keys.id;


-- --
-- -- Name: users; Type: TABLE; Schema: public; Owner: api
-- --

-- CREATE TABLE public.users (
--     id integer NOT NULL,
--     username character varying(40) NOT NULL,
--     email character varying(100) NOT NULL,
--     password character varying(255) NOT NULL,
--     is_admin boolean NOT NULL,
--     email_nonce character varying(255),
--     created_at timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
--     updated_at timestamp(3) without time zone NOT NULL
-- );


-- ALTER TABLE public.users OWNER TO api;

-- --
-- -- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: api
-- --

-- CREATE SEQUENCE public.users_id_seq
--     AS integer
--     START WITH 1
--     INCREMENT BY 1
--     NO MINVALUE
--     NO MAXVALUE
--     CACHE 1;


-- ALTER TABLE public.users_id_seq OWNER TO api;

-- --
-- -- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: api
-- --

-- ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


-- --
-- -- Name: deployments id; Type: DEFAULT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.deployments ALTER COLUMN id SET DEFAULT nextval('public.deployments_id_seq'::regclass);


-- --
-- -- Name: projects id; Type: DEFAULT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.projects ALTER COLUMN id SET DEFAULT nextval('public.projects_id_seq'::regclass);


-- --
-- -- Name: ssh_keys id; Type: DEFAULT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.ssh_keys ALTER COLUMN id SET DEFAULT nextval('public.ssh_keys_id_seq'::regclass);


-- --
-- -- Name: users id; Type: DEFAULT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


-- --
-- -- Data for Name: _prisma_migrations; Type: TABLE DATA; Schema: public; Owner: api
-- --

-- COPY public._prisma_migrations (id, checksum, finished_at, migration_name, logs, rolled_back_at, started_at, applied_steps_count) FROM stdin;
-- 005b0c60-c8c0-4dd9-b6dc-2faf4c837a73	cba3d797f8af16000473cd70d6527bfa327fb2baee351bbb2ca8919cbd24742b	2023-06-26 20:40:17.583066+00	20230626081450_init	\N	\N	2023-06-26 20:40:17.49435+00	1
-- \.


-- --
-- -- Data for Name: deployments; Type: TABLE DATA; Schema: public; Owner: api
-- --

-- COPY public.deployments (id, uuid, name, config, created_at, updated_at, project_id) FROM stdin;
-- \.


-- --
-- -- Data for Name: projects; Type: TABLE DATA; Schema: public; Owner: api
-- --

-- COPY public.projects (id, uuid, name, created_at, updated_at, user_id) FROM stdin;
-- \.


-- --
-- -- Data for Name: ssh_keys; Type: TABLE DATA; Schema: public; Owner: api
-- --

-- COPY public.ssh_keys (id, value, created_at, updated_at, user_id) FROM stdin;
-- \.


-- --
-- -- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: api
-- --

-- COPY public.users (id, username, email, password, is_admin, email_nonce, created_at, updated_at) FROM stdin;
-- \.


-- --
-- -- Name: deployments_id_seq; Type: SEQUENCE SET; Schema: public; Owner: api
-- --

-- SELECT pg_catalog.setval('public.deployments_id_seq', 1, false);


-- --
-- -- Name: projects_id_seq; Type: SEQUENCE SET; Schema: public; Owner: api
-- --

-- SELECT pg_catalog.setval('public.projects_id_seq', 1, false);


-- --
-- -- Name: ssh_keys_id_seq; Type: SEQUENCE SET; Schema: public; Owner: api
-- --

-- SELECT pg_catalog.setval('public.ssh_keys_id_seq', 1, false);


-- --
-- -- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: api
-- --

-- SELECT pg_catalog.setval('public.users_id_seq', 1, false);


-- --
-- -- Name: _prisma_migrations _prisma_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public._prisma_migrations
--     ADD CONSTRAINT _prisma_migrations_pkey PRIMARY KEY (id);


-- --
-- -- Name: deployments deployments_pkey; Type: CONSTRAINT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.deployments
--     ADD CONSTRAINT deployments_pkey PRIMARY KEY (id);


-- --
-- -- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.projects
--     ADD CONSTRAINT projects_pkey PRIMARY KEY (id);


-- --
-- -- Name: ssh_keys ssh_keys_pkey; Type: CONSTRAINT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.ssh_keys
--     ADD CONSTRAINT ssh_keys_pkey PRIMARY KEY (id);


-- --
-- -- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.users
--     ADD CONSTRAINT users_pkey PRIMARY KEY (id);


-- --
-- -- Name: deployments_project_id_key; Type: INDEX; Schema: public; Owner: api
-- --

-- CREATE UNIQUE INDEX deployments_project_id_key ON public.deployments USING btree (project_id);


-- --
-- -- Name: deployments_uuid_key; Type: INDEX; Schema: public; Owner: api
-- --

-- CREATE UNIQUE INDEX deployments_uuid_key ON public.deployments USING btree (uuid);


-- --
-- -- Name: projects_user_id_key; Type: INDEX; Schema: public; Owner: api
-- --

-- CREATE UNIQUE INDEX projects_user_id_key ON public.projects USING btree (user_id);


-- --
-- -- Name: projects_uuid_key; Type: INDEX; Schema: public; Owner: api
-- --

-- CREATE UNIQUE INDEX projects_uuid_key ON public.projects USING btree (uuid);


-- --
-- -- Name: ssh_keys_user_id_key; Type: INDEX; Schema: public; Owner: api
-- --

-- CREATE UNIQUE INDEX ssh_keys_user_id_key ON public.ssh_keys USING btree (user_id);


-- --
-- -- Name: users_email_key; Type: INDEX; Schema: public; Owner: api
-- --

-- CREATE UNIQUE INDEX users_email_key ON public.users USING btree (email);


-- --
-- -- Name: users_username_key; Type: INDEX; Schema: public; Owner: api
-- --

-- CREATE UNIQUE INDEX users_username_key ON public.users USING btree (username);


-- --
-- -- Name: deployments deployments_project_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.deployments
--     ADD CONSTRAINT deployments_project_id_fkey FOREIGN KEY (project_id) REFERENCES public.projects(id) ON UPDATE CASCADE ON DELETE RESTRICT;


-- --
-- -- Name: projects projects_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.projects
--     ADD CONSTRAINT projects_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON UPDATE CASCADE ON DELETE RESTRICT;


-- --
-- -- Name: ssh_keys ssh_keys_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: api
-- --

-- ALTER TABLE ONLY public.ssh_keys
--     ADD CONSTRAINT ssh_keys_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON UPDATE CASCADE ON DELETE RESTRICT;


-- --
-- -- PostgreSQL database dump complete
-- --

