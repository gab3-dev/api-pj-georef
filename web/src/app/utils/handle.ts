export function parsePracaJson(data: any): Object {
    data.codigo = parseInt(data.codigo);
    data.longitude = parseInt(data.longitude);
    data.latitude = parseInt(data.latitude);
    data.codigo_operadora = parseInt(data.codigo_operadora);
    data.km = parseInt(data.km);
    data.codigo_praca = parseInt(data.codigo_praca);
    if (data.cobranca_especial === 'true')
      data.cobranca_especial = true;
    else
      data.cobranca_especial = false;
    
    return data;
}

export function parseOperadoraJson(data: any): Object {
  data.codigo_operadora = parseInt(data.codigo_operadora);
  
  return data;
}