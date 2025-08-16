# Upload de Imagens de Produtos

Este sistema permite o upload de múltiplas imagens para produtos, com suporte a campos adicionais como texto alternativo, posição, status ativo e imagem de capa.

## Funcionalidades

### Frontend (Vue.js)
- **Upload múltiplo**: Suporte para selecionar várias imagens de uma vez
- **Drag & Drop**: Área para arrastar e soltar imagens
- **Preview**: Visualização das imagens antes do upload
- **Campos por imagem**:
  - Arquivo de imagem (file)
  - Texto alternativo (alt_text)
  - Posição (position)
  - Status ativo (active)
  - Imagem de capa (cover)
- **Validação**: Apenas uma imagem pode ser marcada como capa
- **Edição**: Suporte para editar produtos existentes com imagens

### Backend (Rust/Loco)
- **Upload multipart**: Recebe dados do produto e imagens via FormData
- **Processamento de arquivos**: Salva imagens no diretório `uploads/products/`
- **Nomes únicos**: Gera nomes únicos para evitar conflitos
- **Banco de dados**: Salva o caminho relativo das imagens na tabela `product_images`
- **Relacionamentos**: Produtos e imagens são relacionados via `product_id`

## Estrutura do Banco de Dados

### Tabela `product_images`
```sql
CREATE TABLE product_images (
    id INTEGER PRIMARY KEY,
    image VARCHAR,           -- Caminho relativo do arquivo (ex: uuid_0.jpg)
    alt_text VARCHAR,        -- Texto alternativo
    active BOOLEAN,          -- Status ativo
    cover BOOLEAN,           -- É imagem de capa
    position INTEGER,        -- Ordem das imagens
    product_id INTEGER,      -- ID do produto
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
```

## Como Usar

### 1. Criar Novo Produto com Imagens
1. Preencha os dados básicos do produto
2. Use o botão "Adicionar Campo" ou arraste imagens para a área de drop
3. Para cada imagem, configure:
   - **Arquivo**: Selecione o arquivo de imagem
   - **Texto alternativo**: Descrição da imagem para acessibilidade
   - **Posição**: Ordem de exibição (0 = primeira)
   - **Ativa**: Se a imagem deve ser exibida
   - **Capa**: Se é a imagem principal (apenas uma pode ser capa)
4. Clique em "Salvar Produto"

### 2. Editar Produto Existente
1. As imagens existentes serão carregadas automaticamente
2. Você pode:
   - Remover imagens existentes
   - Adicionar novas imagens
   - Modificar metadados das imagens
   - Alterar qual imagem é a capa

### 3. API Endpoints

#### POST `/api/products`
- **Content-Type**: `multipart/form-data`
- **Campos**:
  - `name`, `slug`, `sku`, etc. (dados do produto)
  - `image0`, `image1`, `image2`, etc. (arquivos de imagem)

#### GET `/api/products`
- Retorna produtos com categorias e imagens

#### GET `/api/products/{id}`
- Retorna produto específico com imagens

## Configuração

### Diretório de Upload
As imagens são salvas em `uploads/products/` no servidor. Certifique-se de que:
- O diretório existe e tem permissões de escrita
- O servidor web pode servir arquivos estáticos deste diretório

### Tamanho de Arquivo
Configure o limite de upload no servidor web (nginx, Apache) e no backend Rust.

### Tipos de Arquivo
O sistema aceita apenas arquivos de imagem (`image/*`).

## Exemplo de Uso

```typescript
// Frontend - Enviar produto com imagens
const formData = new FormData()
formData.append('name', 'Produto Exemplo')
formData.append('price', '99.99')
formData.append('image0', imageFile1)
formData.append('image1', imageFile2)

const response = await fetch('/api/products', {
  method: 'POST',
  body: formData
})
```

```rust
// Backend - Processar upload
pub async fn add(State(ctx): State<AppContext>, mut multipart: Multipart) -> Result<Response> {
    let mut product = ActiveModel { ..Default::default() };
    let mut images = Vec::new();
    
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        if name.starts_with("image") {
            // Processar imagem
            let image_model = save_image(field, product_id, position).await?;
            images.push(image_model);
        } else {
            // Processar dados do produto
            // ...
        }
    }
    
    // Salvar produto e imagens
    // ...
}

async fn save_image(field: Field<'_>, product_id: i32, position: i32) -> Result<ProductImageActiveModel> {
    let filename = field.file_name().unwrap_or("image.jpg").to_string();
    let extension = Path::new(&filename).extension().and_then(|ext| ext.to_str()).unwrap_or("jpg");
    let unique_name = format!("{}_{}.{}", Uuid::new_v4(), position, extension);
    
    let upload_dir = "uploads/products";
    fs::create_dir_all(upload_dir).await?;
    
    let data = field.bytes().await?;
    fs::write(format!("{}/{}", upload_dir, unique_name), &data).await?;
    
    Ok(ProductImageActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        image: sea_orm::ActiveValue::Set(Some(unique_name)), // Caminho relativo salvo no banco
        alt_text: sea_orm::ActiveValue::Set(Some(filename)),
        active: sea_orm::ActiveValue::Set(Some(true)),
        cover: sea_orm::ActiveValue::Set(Some(position == 0)),
        position: sea_orm::ActiveValue::Set(Some(position)),
        product_id: sea_orm::ActiveValue::Set(product_id),
        created_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
        updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
    })
}
```

## Considerações de Segurança

1. **Validação de tipo**: Apenas arquivos de imagem são aceitos
2. **Nomes únicos**: Evita sobrescrita de arquivos
3. **Sanitização**: Remove caracteres especiais dos nomes de arquivo
4. **Limite de tamanho**: Configure limites apropriados no servidor

## Próximos Passos

- [ ] Suporte para redimensionamento de imagens
- [ ] Compressão automática
- [ ] Suporte para diferentes formatos (WebP, AVIF)
- [ ] CDN para imagens
- [ ] Backup automático das imagens 