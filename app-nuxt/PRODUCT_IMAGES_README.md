# Product Image Upload System

This system allows uploading multiple images for products, with support for additional fields such as alternative text, position, active status, and cover image.

## Features

### Frontend (Vue.js)
- **Multiple upload**: Support for selecting multiple images at once
- **Drag & Drop**: Area for dragging and dropping images
- **Preview**: Image preview before upload
- **Per-image fields**:
  - Image file (file)
  - Alternative text (alt_text)
  - Position (position)
  - Active status (active)
  - Cover image (cover)
- **Validation**: Only one image can be marked as cover
- **Editing**: Support for editing existing products with images

### Backend (Rust/Loco)
- **Multipart upload**: Receives product data and images via FormData
- **File processing**: Saves images in the `uploads/products/` directory
- **Unique names**: Generates unique names to avoid conflicts
- **Database**: Saves the relative path of images in the `product_images` table
- **Relationships**: Products and images are related via `product_id`

## Database Structure

### `product_images` Table
```sql
CREATE TABLE product_images (
    id INTEGER PRIMARY KEY,
    image VARCHAR,           -- Relative file path (e.g., uuid_0.jpg)
    alt_text VARCHAR,        -- Alternative text
    active BOOLEAN,          -- Active status
    cover BOOLEAN,           -- Is cover image
    position INTEGER,        -- Image order
    product_id INTEGER,      -- Product ID
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
```

## How to Use

### 1. Create New Product with Images
1. Fill in the basic product data
2. Use the "Add Field" button or drag images to the drop area
3. For each image, configure:
   - **File**: Select the image file
   - **Alternative text**: Image description for accessibility
   - **Position**: Display order (0 = first)
   - **Active**: Whether the image should be displayed
   - **Cover**: If it's the main image (only one can be cover)
4. Click "Save Product"

### 2. Edit Existing Product
1. Existing images will be loaded automatically
2. You can:
   - Remove existing images
   - Add new images
   - Modify image metadata
   - Change which image is the cover

### 3. API Endpoints

#### POST `/api/products`
- **Content-Type**: `multipart/form-data`
- **Fields**:
  - `name`, `slug`, `sku`, etc. (product data)
  - `image0`, `image1`, `image2`, etc. (image files)

#### GET `/api/products`
- Returns products with categories and images

#### GET `/api/products/{id}`
- Returns specific product with images

## Configuration

### Upload Directory
Images are saved in `uploads/products/` on the server. Make sure that:
- The directory exists and has write permissions
- The web server can serve static files from this directory

### File Size
Configure upload limits in the web server (nginx, Apache) and Rust backend.

### File Types
The system only accepts image files (`image/*`).

## Usage Example

```typescript
// Frontend - Send product with images
const formData = new FormData()
formData.append('name', 'Example Product')
formData.append('price', '99.99')
formData.append('image0', imageFile1)
formData.append('image1', imageFile2)

const response = await fetch('/api/products', {
  method: 'POST',
  body: formData
})
```

```rust
// Backend - Process upload
pub async fn add(State(ctx): State<AppContext>, mut multipart: Multipart) -> Result<Response> {
    let mut product = ActiveModel { ..Default::default() };
    let mut images = Vec::new();
    
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        if name.starts_with("image") {
            // Process image
            let image_model = save_image(field, product_id, position).await?;
            images.push(image_model);
        } else {
            // Process product data
            // ...
        }
    }
    
    // Save product and images
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
        image: sea_orm::ActiveValue::Set(Some(unique_name)), // Relative path saved in database
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

## Security Considerations

1. **Type validation**: Only image files are accepted
2. **Unique names**: Prevents file overwriting
3. **Sanitization**: Removes special characters from filenames
4. **Size limits**: Configure appropriate limits on the server

## Next Steps

- [ ] Support for image resizing
- [ ] Automatic compression
- [ ] Support for different formats (WebP, AVIF)
- [ ] CDN for images
- [ ] Automatic image backup 