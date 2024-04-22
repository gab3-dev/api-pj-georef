CREATE DATABASE IF NOT EXISTS pj_georef;

USE pj_georef;  

CREATE TABLE operadora (
    id VARCHAR(255),
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

CREATE TABLE praca (
    id VARCHAR(255),
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
    codigo_praca TINYINT,
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