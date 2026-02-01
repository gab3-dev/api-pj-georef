# CSV Test Files Documentation

This directory contains comprehensive CSV test files for testing the import functionality of Operadoras, Pedágios, and Tarifas in the API PJ GeoRef system.

## Overview

The test files are designed to cover:
- **Typical use cases**: Valid data with standard values
- **Missing fields**: Testing optional and nullable fields
- **Invalid data**: Testing error handling for invalid values
- **Edge cases**: Boundary values, special characters, formatting variations
- **Volume testing**: Large datasets to test performance

## File Structure

### Operadoras Test Files

| File | Purpose | Records | Description |
|------|---------|---------|-------------|
| `test_operadoras_valid.csv` | Valid data | 5 | Standard operadora records with all required fields properly filled |
| `test_operadoras_missing_fields.csv` | Missing optional fields | 5 | Tests nullable fields like CNPJ, email, telefone |
| `test_operadoras_invalid_fields.csv` | Invalid data | 5 | Tests invalid CNPJ formats, email formats, dates, and field lengths |
| `test_operadoras_edge_cases.csv` | Edge cases | 5 | Tests special characters, accents, quotes, empty strings |
| `test_operadoras_large_volume.csv` | Volume testing | 100 | Large dataset for performance and bulk import testing |

**CSV Format:**
```
codigo_operadora;operadora;CNPJ;razao_social;data_alteracao;email;telefone;grupo;responsavel
```

**Field Details:**
- `codigo_operadora`: Integer, UNIQUE, required
- `operadora`: VARCHAR(255), required
- `CNPJ`: VARCHAR(30), optional
- `razao_social`: VARCHAR(100), optional
- `data_alteracao`: VARCHAR(11), format: DD/MM/YYYY
- `email`: VARCHAR(100), optional
- `telefone`: VARCHAR(30), optional
- `grupo`: VARCHAR(100), required
- `responsavel`: VARCHAR(100), required

### Pedágios Test Files

| File | Purpose | Records | Description |
|------|---------|---------|-------------|
| `test_pedagios_valid.csv` | Valid data | 5 | Standard pedagio records with proper coordinates and references |
| `test_pedagios_missing_fields.csv` | Missing optional fields | 5 | Tests nullable fields like codigo, orientacao, email, telefone |
| `test_pedagios_invalid_fields.csv` | Invalid data | 5 | Tests invalid foreign keys, coordinates, and boolean values |
| `test_pedagios_edge_cases.csv` | Edge cases | 5 | Tests extreme coordinates, special characters, empty names |
| `test_pedagios_large_volume.csv` | Volume testing | 100 | Large dataset for performance testing |

**CSV Format:**
```
id_pedagio;longitude;latitude;nome;codigo_operadora;concessionaria;situacao;sigla;rodovia;km;id_trecho;sentido;cidade;estado;codigo;orientacao;tipo;jurisdicao;cobranca_especial;categoria;data_alteracao;razao_social;cnpj;email;telefone
```

**Field Details:**
- `id_pedagio`: INT, PRIMARY KEY, required
- `longitude`: BIGINT, required (large negative integers for coordinates)
- `latitude`: BIGINT, required (large negative integers for coordinates)
- `nome`: VARCHAR(50), required
- `codigo_operadora`: INT, FOREIGN KEY to operadora, required
- `concessionaria`: VARCHAR(100), concession name, required
- `situacao`: VARCHAR(20), typical values: "Ativo", "Desativado"
- `sigla`: VARCHAR(20), abbreviation/acronym, required
- `rodovia`: VARCHAR(100), optional
- `km`: DOUBLE PRECISION, kilometer position
- `id_trecho`: INT, road segment identifier, required
- `sentido`: VARCHAR(10), direction (Norte, Sul, Leste, Oeste)
- `cidade`: VARCHAR(50), optional
- `estado`: VARCHAR(3), state abbreviation
- `codigo`: VARCHAR(50), optional
- `orientacao`: VARCHAR(100), optional
- `tipo`: VARCHAR(40), typical values: "PED", "BAL"
- `jurisdicao`: VARCHAR(20), typical values: "Federal", "Estadual"
- `cobranca_especial`: BOOLEAN (0 or 1)
- `categoria`: VARCHAR(3), optional
- `data_alteracao`: VARCHAR(11), format: DD/MM/YYYY
- `razao_social`: VARCHAR(100), optional
- `cnpj`: VARCHAR(30), optional
- `email`: VARCHAR(100), optional
- `telefone`: VARCHAR(30), optional

### Tarifas Test Files

| File | Purpose | Records | Description |
|------|---------|---------|-------------|
| `test_tarifas_valid.csv` | Valid data | 10 | Standard tarifa records with proper foreign key references |
| `test_tarifas_missing_fields.csv` | Missing optional fields | 6 | Tests nullable fields and default values |
| `test_tarifas_invalid_fields.csv` | Invalid data | 8 | Tests invalid foreign keys, negative values, invalid formats |
| `test_tarifas_edge_cases.csv` | Edge cases | 7 | Tests zero values, extreme values, boundary timestamps |
| `test_tarifas_large_volume.csv` | Volume testing | 100 | Large dataset for performance testing |

**CSV Format:**
```
id_tarifa;id_tipo_tarifa;id_pedagio;multiplicador;valor;data_criacao;data_atualizacao;situacao;tipo
```

**Field Details:**
- `id_tarifa`: INT, PRIMARY KEY, required
- `id_tipo_tarifa`: INT, FOREIGN KEY to tipo_tarifa, required
- `id_pedagio`: INT, FOREIGN KEY to pedagio, required
- `multiplicador`: DOUBLE PRECISION, required
- `valor`: DOUBLE PRECISION, tariff value in BRL, required
- `data_criacao`: TIMESTAMP, format: YYYY-MM-DD HH:MM:SS, required
- `data_atualizacao`: TIMESTAMP, format: YYYY-MM-DD HH:MM:SS, required
- `situacao`: VARCHAR(20), typical values: "Ativo", "Inativo"
- `tipo`: VARCHAR(20), typical values: "Normal", "Especial"

## Usage

### Testing with Valid Data

```bash
# Upload valid operadoras
curl -X POST http://localhost:9999/api/importar-operadoras \
  -F "file=@tmp/test_operadoras_valid.csv"

# Upload valid pedagios
curl -X POST http://localhost:9999/api/importar-pedagios \
  -F "file=@tmp/test_pedagios_valid.csv"

# Upload valid tarifas
curl -X POST http://localhost:9999/api/importar-tarifas \
  -F "file=@tmp/test_tarifas_valid.csv"
```

### Testing Error Handling

Use the `invalid_fields` and `missing_fields` files to test error handling:

```bash
# Test invalid operadora data
curl -X POST http://localhost:9999/api/importar-operadoras \
  -F "file=@tmp/test_operadoras_invalid_fields.csv"
```

### Testing Edge Cases

Use the `edge_cases` files to test special scenarios:

```bash
# Test operadoras with special characters
curl -X POST http://localhost:9999/api/importar-operadoras \
  -F "file=@tmp/test_operadoras_edge_cases.csv"
```

### Volume Testing

Use the `large_volume` files to test performance:

```bash
# Test bulk import of 100 operadoras
curl -X POST http://localhost:9999/api/importar-operadoras \
  -F "file=@tmp/test_operadoras_large_volume.csv"
```

## Import API Details

### Common Parameters
- **Delimiter**: `;` (semicolon)
- **Encoding**: UTF8 (operadoras uses ISO88599)
- **Headers**: CSV HEADER (first line contains column names)

### Expected Responses

**Success:**
```json
{
  "operadoras_importadas": 5
}
```

**Duplicate Key Error:**
```json
{
  "operadoras_importadas": 0,
  "erro": "operadoras já importadas",
  "detalhes": "Código da operadora 100 já existe."
}
```

**General Error:**
```json
{
  "erro": "Error message details"
}
```

## Test Scenarios Covered

### Operadoras
- ✅ Valid complete records
- ✅ Missing optional fields (CNPJ, email, telefone)
- ✅ Invalid CNPJ format
- ✅ Invalid email format
- ✅ Invalid date format
- ✅ Special characters and accents (ã, ç, é, etc.)
- ✅ Quoted strings
- ✅ Empty strings
- ✅ Very long names (field length limits)
- ✅ Bulk import (100+ records)

### Pedágios
- ✅ Valid complete records with all relationships
- ✅ Missing optional fields (codigo, orientacao, email, telefone)
- ✅ Invalid foreign key (codigo_operadora)
- ✅ Invalid coordinates (out of range)
- ✅ Invalid boolean values
- ✅ Negative kilometer values
- ✅ Extreme coordinate values
- ✅ Zero coordinates
- ✅ Special characters in names and locations
- ✅ Empty required fields
- ✅ Bulk import (100+ records)

### Tarifas
- ✅ Valid complete records with proper references
- ✅ Missing optional fields (tipo, situacao)
- ✅ Missing nullable fields (multiplicador, valor)
- ✅ Invalid foreign keys (id_tipo_tarifa, id_pedagio)
- ✅ Negative values for multiplicador and valor
- ✅ Invalid date/timestamp formats
- ✅ Zero values for amounts
- ✅ Very large values (edge of numeric limits)
- ✅ Very small decimal values (0.01)
- ✅ Boundary timestamps (1970, 2099)
- ✅ Bulk import (100+ records)

## Notes

1. **Foreign Key Dependencies**: When testing, ensure operadoras are imported before pedagios, and both are imported before tarifas due to foreign key constraints.

2. **ID Ranges**: Test files use specific ID ranges to avoid conflicts:
   - Operadoras valid: 100-104
   - Operadoras missing fields: 200-204
   - Operadoras invalid: 300-304
   - Operadoras edge cases: 400-404
   - Operadoras large volume: 500-599
   - Pedagios valid: Uses existing operadora IDs (1, 2, 3)
   - Tarifas valid: 1000-1009
   - Tarifas missing fields: 2000-2005
   - Tarifas invalid: 3000-3007
   - Tarifas edge cases: 4000-4006
   - Tarifas large volume: 5000-5099

3. **Encoding**: The operadoras import uses ISO88599 encoding, while pedagios and tarifas use UTF8. Ensure your test files are properly encoded.

4. **Cleanup**: After testing, you may need to clean up test data from the database to avoid conflicts in subsequent tests.

## Integration Testing Example

```bash
#!/bin/bash

# Complete integration test workflow

# 1. Import operadoras (dependencies first)
echo "Importing operadoras..."
curl -X POST http://localhost:9999/api/importar-operadoras \
  -F "file=@tmp/test_operadoras_valid.csv"

# 2. Import pedagios (depends on operadoras)
echo "Importing pedagios..."
curl -X POST http://localhost:9999/api/importar-pedagios \
  -F "file=@tmp/test_pedagios_valid.csv"

# 3. Import tarifas (depends on pedagios and tipo_tarifa)
echo "Importing tarifas..."
curl -X POST http://localhost:9999/api/importar-tarifas \
  -F "file=@tmp/test_tarifas_valid.csv"

echo "Import complete!"
```

## Troubleshooting

### Import Fails with Permission Error
Ensure the CSV files have proper permissions (0664) and the ./tmp directory is mounted correctly in Docker.

### Foreign Key Constraint Violations
Verify that parent records exist before importing child records. Check the test data uses valid references.

### Encoding Issues
If you see garbled characters (�), the file encoding may not match the import encoding setting. Check if the file is UTF8 or ISO88599.

### Duplicate Key Errors
The import API prevents duplicate keys. Use different ID ranges or clean up existing test data before re-running tests.
