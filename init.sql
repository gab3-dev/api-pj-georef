CREATE TABLE IF NOT EXISTS operadora (
    id SERIAL PRIMARY KEY,
    data_operacao VARCHAR(100),
    responsavel VARCHAR(100),
    grupo VARCHAR(100),
    codigo_operadora INT,
    operadora VARCHAR(255),
    razao_social VARCHAR(100),
    cnpj VARCHAR(100),
    email VARCHAR(100),
    telefone VARCHAR(100)
);

-- INSERT INTO operadora (data_operacao, responsavel, grupo, codigo_operadora, operadora, razao_social, cnpj, email, telefone)
-- VALUES ('2022-01-01', 'John Doe', 'Group A', 123, 'ABC Telecom', 'ABC Company', '1234567890', 'john.doe@example.com', '123-456-7890');

CREATE TABLE IF NOT EXISTS praca (
    id SERIAL PRIMARY KEY,
    longitude INT,
    latitude INT,
    id_operadora VARCHAR(100),
    nome VARCHAR(255),
    situacao VARCHAR(100),
    rodovia VARCHAR(100),
    km INT,
    sentido VARCHAR(100),
    cidade VARCHAR(100),
    estado VARCHAR(100),
    codigo_praca SMALLINT,
    orientacao VARCHAR(100),
    tipo VARCHAR(100),
    jurisdicao VARCHAR(100),
    cobranca_especial BOOLEAN,
    categoria VARCHAR(100),
    data_de_alteracao VARCHAR(100),
    razao_social VARCHAR(100),
    cnpj VARCHAR(100),
    email VARCHAR(100),
    telefone VARCHAR(100)
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
--     'codigo_praca', codigo_praca,
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
-- ) FROM praca;