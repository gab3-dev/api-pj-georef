import { expect, test } from '@playwright/test';
import { Client } from 'pg';

const adminEmail = process.env['ADMIN_EMAIL'] ?? 'admin@bgm.com';
const adminPassword = process.env['ADMIN_PASSWORD'] ?? 'e2e_admin_password';
const apiUrl = process.env['API_URL'] ?? 'http://localhost:8081/api';

function dbClient() {
  return new Client({
    host: process.env['DB_HOST'] ?? 'localhost',
    port: Number(process.env['DB_PORT'] ?? 5432),
    user: process.env['POSTGRES_USER'] ?? 'root',
    password: process.env['POSTGRES_PASSWORD'] ?? '1234',
    database: process.env['POSTGRES_DB'] ?? 'pj_georef',
  });
}

test('login, cadastro e persistencia de operadora', async ({ page, request }) => {
  const timestamp = Date.now();
  const codigoOperadora = Number(String(timestamp).slice(-8));
  const nomeOperadora = `Operadora E2E ${codigoOperadora}`;
  const client = dbClient();

  await client.connect();

  try {
    await page.goto('/login');
    await page.getByPlaceholder('Email').fill(adminEmail);
    await page.getByPlaceholder('Senha').fill(adminPassword);
    await page.getByRole('button', { name: 'Entrar' }).click();

    await expect(page).toHaveURL(/\/$/);

    await page.goto('/operadoras');
    await page.getByRole('tab', { name: 'Cadastrar Operadora' }).click();

    await page.locator('input[name="data_alteracao"]').fill('2026-04-29');
    await page.locator('input[name="responsavel"]').fill('Responsavel E2E');
    await page.locator('input[name="grupo"]').fill('Grupo E2E');
    await page.locator('input[name="codigo_operadora"]').fill(String(codigoOperadora));
    await page.locator('input[name="operadora"]').fill(nomeOperadora);
    await page.locator('input[name="razao_social"]').fill(`Razao Social E2E ${codigoOperadora}`);
    await page.locator('input[name="cnpj"]').fill('12.345.678/0001-90');
    await page.locator('input[name="email"]').fill(`operadora-${codigoOperadora}@e2e.test`);
    await page.locator('input[name="telefone"]').fill('(11) 99999-9999');

    const dialogPromise = page.waitForEvent('dialog');
    await page.getByRole('button', { name: 'Enviar' }).click();
    const dialog = await dialogPromise;
    expect(dialog.message()).toContain('Operadora inserida com sucesso');
    await dialog.accept();

    await page.reload();
    await page.getByRole('tab', { name: 'Listar Operadoras' }).click();
    await expect(page.getByText(nomeOperadora)).toBeVisible();

    const token = await page.evaluate(() => window.localStorage.getItem('token'));
    expect(token).toBeTruthy();

    const response = await request.get(`${apiUrl}/get-operadora/${codigoOperadora}`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    expect(response.ok()).toBeTruthy();
    const operadoras = await response.json();
    expect(operadoras).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          codigo_operadora: codigoOperadora,
          operadora: nomeOperadora,
        }),
      ]),
    );

    const result = await client.query(
      'SELECT codigo_operadora, operadora FROM operadora WHERE codigo_operadora = $1',
      [codigoOperadora],
    );
    expect(result.rowCount).toBe(1);
    expect(result.rows[0]).toMatchObject({
      codigo_operadora: codigoOperadora,
      operadora: nomeOperadora,
    });
  } finally {
    await client.query('DELETE FROM operadora WHERE codigo_operadora = $1', [codigoOperadora]);
    await client.end();
  }
});
