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

INSERT INTO operadora (data_alteracao, responsavel, grupo, codigo_operadora, operadora, razao_social, cnpj, email, telefone)
VALUES
    ('2021-06-01', 'Fernando', 'Grupo 1', 1, 'EcoRodovias', 'EcoRodovias Infraestrutura e Logística S.A.', '59.075.281/0001-28', 'email@teste.com', '(11) 99999-9999'),
    ('2021-06-01', 'Fernando', 'Grupo 1', 2, 'CCR', 'CCR S.A.', '02.846.056/0001-97', 'email@teste.com', '(11) 99999-9999'),
    ('2021-06-01', 'Fernando', 'Grupo 1', 3, 'Arteris', 'Arteris S.A.', '02.919.555/0001-67', 'email@teste.com', '(11) 99999-9999');

CREATE TABLE IF NOT EXISTS pedagio (
    id_pedagio INT,
    longitude BIGINT,
    latitude BIGINT,
    nome VARCHAR(50),
    codigo_operadora INT,
    concessionaria VARCHAR(100),
    situacao VARCHAR(20),
    sigla VARCHAR(20),
    rodovia VARCHAR(100),
    km DOUBLE PRECISION,
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

INSERT INTO pedagio (id_pedagio, longitude, latitude, nome, codigo_operadora, concessionaria, situacao, sigla, rodovia, km, id_trecho, sentido, cidade, estado, codigo, orientacao, tipo, jurisdicao, cobranca_especial, categoria, data_alteracao, razao_social, cnpj, email, telefone)
VALUES
    (1, 1, 1, 'Pedágio 1', 1, 'EcoRodovias', 'Ativo', 'ECO', 'BR-116', 1.0, 1, 'Norte', 'São Paulo', 'SP', '0001', 'Km 1', 'Praça de Pedágio', 'Concedida', FALSE, 'A', '2021-06-01', 'EcoRodovias', '59.075.281/0001-28', 'email@test.com', '(11) 99999-9999'),
    (2, 2, 2, 'Pedágio 2', 2, 'CCR', 'Ativo', 'CCR', 'BR-116', 2.0, 2, 'Sul', 'São Paulo', 'SP', '0002', 'Km 2', 'Praça de Pedágio', 'Concedida', FALSE, 'A', '2021-06-01', 'CCR', '02.846.056/0001-97', 'email@test.com', '(11) 99999-9999'),
    (3, 3, 3, 'Pedágio 3', 3, 'Arteris', 'Ativo', 'ART', 'BR-116', 3.0, 3, 'Norte', 'São Paulo', 'SP', '0003', 'Km 3', 'Praça de Pedágio', 'Concedida', FALSE, 'A', '2021-06-01', 'Arteris', '02.919.555/0001-67', 'email@test.com', '(11) 99999-9999');


CREATE TABLE IF NOT EXISTS tipo_tarifa (
    id_tipo_tarifa INT NOT NULL PRIMARY KEY,
    id_padrao_tarifa INT NULL,
    descricao VARCHAR(200),
    tipo_rodagem INT,
    rodagem VARCHAR(10),
    eixos INT
);

INSERT INTO tipo_tarifa (id_tipo_tarifa, id_padrao_tarifa, descricao, tipo_rodagem, rodagem, eixos)
VALUES
    (1, 1, 'Automóvel, Caminhonete, Caminhoneta e Furgão', 1, 'Simples', 2),
    (2, NULL, 'Automóvel, Caminhoneta e Furgão', 2, 'Dupla', 2),
    (3, 2, 'Caminhão Leve, Caminhão-Trator e Furgão', 2, 'Dupla', 2),
    (4, NULL, 'Onibus', 2, 'Dupla', 2),
    (5, 7, 'Automóvel, Caminhonete e Caminhoneta com semireboque', 1, 'Simples', 3),
    (6, 3, 'Caminhão, Caminhão-Trator, Caminhão-Trator com semireboque', 2, 'Dupla', 3),
    (7, NULL, 'Onibus', 2, 'Dupla', 3),
    (8, 8, 'Automóvel e Caminhoneta ou Caminhonete com reboque', 1, 'Simples', 4),
    (9, 4, 'Caminhão e/ou Caminhão-Trator com semirreboque', 2, 'Dupla', 4),
    (10, 5, 'Caminhão com reboque e Caminhão-Trator', 2, 'Dupla', 5),
    (11, 6, 'Caminhão com reboque e Caminhão-Trator com semirreboque', 2, 'Dupla', 6),
    (12, 10, 'Caminhão com reboque e Caminhão-Trator com semirreboque', 2, 'Dupla', 7),
    (13, NULL, 'Caminhão com reboque e Caminhão-Trator com semirreboque', 2, 'Dupla', 8),
    (14, NULL, 'Caminhão com reboque e Caminhão-Trator com semirreboque', 2, 'Dupla', 9),
    (15, NULL, 'Veículos transportadores de inflamáveis', 2, 'Dupla', 4),
    (16, NULL, 'Carros Forte', 2, 'Dupla', 2),
    (17, 9, 'Motocicleta, motonetas e bicicleta a motor', 1, 'Simples', 2);

CREATE TABLE IF NOT EXISTS tarifas (
    id_tarifa INT NOT NULL PRIMARY KEY,
    id_tipo_tarifa INT NOT NULL,
    id_pedagio INT NOT NULL,
    multiplicador DOUBLE PRECISION,
    valor DOUBLE PRECISION,
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP,
    situacao VARCHAR(20),
    tipo VARCHAR(20),
    CONSTRAINT fk_tipo_tarifa
      FOREIGN KEY(id_tipo_tarifa)
        REFERENCES tipo_tarifa(id_tipo_tarifa),
    CONSTRAINT fk_pedagio
      FOREIGN KEY(id_pedagio)
        REFERENCES pedagio(id_pedagio)
);

-- insert for pedagio 2
INSERT INTO tarifas (id_tarifa, id_tipo_tarifa, id_pedagio, multiplicador, valor, data_criacao, data_atualizacao, situacao, tipo)
VALUES 
    (1, 1, 2, 1.0, 10.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (2, 2, 2, 1.0, 20.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (3, 3, 2, 1.0, 30.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (4, 4, 2, 1.0, 40.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (5, 5, 2, 1.0, 50.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (6, 6, 2, 1.0, 60.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (7, 7, 2, 1.0, 70.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (8, 8, 2, 1.0, 80.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (9, 9, 2, 1.0, 90.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (10, 10, 2, 1.0, 100.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (11, 11, 2, 1.0, 110.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (12, 12, 2, 1.0, 120.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (13, 13, 2, 1.0, 130.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (14, 14, 2, 1.0, 140.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (15, 15, 2, 1.0, 150.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (16, 16, 2, 1.0, 160.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal'),
    (17, 17, 2, 1.0, 170.0, '2021-06-01', '2021-06-01', 'Ativo', 'Normal');

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