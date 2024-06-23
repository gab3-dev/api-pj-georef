CREATE TABLE IF NOT EXISTS operadora (
    id_operadora uuid DEFAULT gen_random_uuid(),
    data_alteracao VARCHAR(11),
    responsavel VARCHAR(100),
    grupo VARCHAR(100),
    codigo_operadora INT UNIQUE,
    operadora VARCHAR(255),
    razao_social VARCHAR(100),
    cnpj VARCHAR(30),
    email VARCHAR(100),
    telefone VARCHAR(30),
    PRIMARY KEY(id_operadora)
);

-- INSERT INTO operadora (data_operacao, responsavel, grupo, codigo_operadora, operadora, razao_social, cnpj, email, telefone)
-- VALUES ('2022-01-01', 'John Doe', 'Group A', 123, 'ABC Telecom', 'ABC Company', '1234567890', 'john.doe@example.com', '123-456-7890');

CREATE TABLE IF NOT EXISTS pedagio (
    id_pedagio INT,
    longitude INT,
    latitude INT,
    nome VARCHAR(50),
    codigo_operadora INT,
    concessionaria VARCHAR(100),
    situacao VARCHAR(20),
    sigla VARCHAR(20),
    rodovia VARCHAR(100),
    km REAL,
    id_trecho INT,
    sentido VARCHAR(10),
    cidade VARCHAR(50),
    estado VARCHAR(3),
    codigo VARCHAR(50),
    orientacao VARCHAR(100),
    tipo VARCHAR(40),
    jurisdicao VARCHAR(20),
    cobranca_especial BOOLEAN,
    categoria VARCHAR(3),
    data_alteracao VARCHAR(11),
    razao_social VARCHAR(100),
    cnpj VARCHAR(30),
    email VARCHAR(100),
    telefone VARCHAR(30),
    PRIMARY KEY(id_pedagio),
    CONSTRAINT fk_operadora
      FOREIGN KEY(codigo_operadora) 
        REFERENCES operadora(codigo_operadora)
);

-- SELECT JSON_OBJECT(
--     'id', id,
--     'longitude', longitude,
--     'latitude', latitude,
--     'id_operadora', id_operadora,
--     'nome', nome,
--     'situacao', situacao,
--     'rodovia', rodovia,
--     'km', km,
--     'sentido', sentido,
--     'cidade', cidade,
--     'estado', estado,
--     'codigo_pedagio', codigo_pedagio,
--     'orientacao', orientacao,
--     'tipo', tipo,
--     'jurisdicao', jurisdicao,
--     'cobranca_especial', cobranca_especial,
--     'categoria', categoria,
--     'data_de_alteracao', data_de_alteracao,
--     'razao_social', razao_social,
--     'cnpj', cnpj,
--     'email', email,
--     'telefone', telefone
-- ) FROM pedagio;