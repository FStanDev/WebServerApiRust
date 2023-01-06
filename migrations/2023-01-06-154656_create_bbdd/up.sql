-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public."Garage"
(
    id bigint NOT NULL,
    name character(50) COLLATE pg_catalog."default" NOT NULL,
    location character(150) COLLATE pg_catalog."default" NOT NULL,
    capacity integer NOT NULL,
    CONSTRAINT "Garage_pkey" PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."Garage"
    OWNER to stantech;

-- Table: public.WorkingCars

-- DROP TABLE IF EXISTS public."WorkingCars";
CREATE TABLE IF NOT EXISTS public."CarToRepair"
(
    id uuid NOT NULL,
    modelo character(50) COLLATE pg_catalog."default" NOT NULL,
    fecha_entrada timestamp with time zone NOT NULL,
    fecha_salida timestamp with time zone NOT NULL,
    CONSTRAINT "CarToRepair_pkey" PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."CarToRepair"
    OWNER to stantech;

CREATE TABLE IF NOT EXISTS public."WorkingCars"
(
    id bigint NOT NULL ,
    assigned_garage bigint NOT NULL ,
    car_to_repair uuid NOT NULL,
    CONSTRAINT "WorkingCars_pkey" PRIMARY KEY (id),
    CONSTRAINT "CocheAsignado" FOREIGN KEY (car_to_repair)
        REFERENCES public."CarToRepair" (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "GarajeAsignado" FOREIGN KEY (assigned_garage)
        REFERENCES public."Garage" (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."WorkingCars"
    OWNER to stantech;