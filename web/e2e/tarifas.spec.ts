import { expect, test } from '@playwright/test';
import { Client } from 'pg';

const adminEmail = process.env['ADMIN_EMAIL'] ?? 'admin@bgm.com';
const adminPassword = process.env['ADMIN_PASSWORD'] ?? 'e2e_admin_password';

function dbClient() {
  return new Client({
    host: process.env['DB_HOST'] ?? 'localhost',
    port: Number(process.env['DB_PORT'] ?? 5432),
    user: process.env['POSTGRES_USER'] ?? 'root',
    password: process.env['POSTGRES_PASSWORD'] ?? '1234',
    database: process.env['POSTGRES_DB'] ?? 'pj_georef',
  });
}

test('atualiza tarifa selecionada a partir do pedagio sem trocar tipo ou pedagio', async ({ page }) => {
  const idTarifa = 130001;
  const client = dbClient();

  await client.connect();

  try {
    await client.query('DELETE FROM tarifas WHERE id_tarifa = $1', [idTarifa]);
    await client.query(
      `DELETE FROM tarifas
       WHERE id_tipo_tarifa = 1
         AND id_pedagio = 1
         AND valor = 10.0
         AND situacao = 'Inativo'
         AND data_criacao = '2024-01-01'`,
    );
    await client.query(
      `INSERT INTO tarifas (
        id_tarifa, id_tipo_tarifa, id_pedagio, multiplicador, valor,
        data_criacao, data_atualizacao, situacao, tipo
      )
      VALUES ($1, 1, 1, 1.0, 10.0, '2024-01-01', '2024-01-01', 'Ativo', 'Normal')`,
      [idTarifa],
    );

    await page.goto('/login');
    await page.getByPlaceholder('Email').fill(adminEmail);
    await page.getByPlaceholder('Senha').fill(adminPassword);
    await page.getByRole('button', { name: 'Entrar' }).click();
    await expect(page).toHaveURL(/\/$/);

    await page.goto('/tarifas');
    await page.getByRole('tab', { name: 'Atualizar Tarifa' }).click();

    await page.getByPlaceholder('Digite o ID, praça ou operadora').fill('Pedágio 1');
    await page.getByRole('option', { name: /1 - Pedágio 1/ }).click();

    await page.getByPlaceholder('Digite o ID, descrição ou tipo').fill(String(idTarifa));
    await page.getByRole('option', { name: new RegExp(`^${idTarifa}\\b`) }).click();

    await page.locator('input[formcontrolname="multiplicador"]').fill('2');
    await page.locator('input[formcontrolname="valor"]').fill('77.7');
    await page.locator('input[formcontrolname="data_criacao"]').fill('2024-01-01');
    await page.locator('input[formcontrolname="situacao"]').fill('Ativo');
    await page.locator('input[formcontrolname="tipo"]').fill('Normal');
    await page.getByRole('button', { name: 'Atualizar Tarifa' }).click();

    await expect(page.getByText('Tarifa atualizada com sucesso!')).toBeVisible();

    const result = await client.query(
      `SELECT id_tipo_tarifa, id_pedagio, multiplicador, valor, situacao, tipo
       FROM tarifas
       WHERE id_tarifa = $1`,
      [idTarifa],
    );

    expect(result.rowCount).toBe(1);
    expect(result.rows[0]).toMatchObject({
      id_tipo_tarifa: 1,
      id_pedagio: 1,
      multiplicador: 2,
      valor: 77.7,
      situacao: 'Ativo',
      tipo: 'Normal',
    });
  } finally {
    await client.query('DELETE FROM tarifas WHERE id_tarifa = $1', [idTarifa]);
    await client.query(
      `DELETE FROM tarifas
       WHERE id_tipo_tarifa = 1
         AND id_pedagio = 1
         AND valor = 10.0
         AND situacao = 'Inativo'
         AND data_criacao = '2024-01-01'`,
    );
    await client.end();
  }
});
