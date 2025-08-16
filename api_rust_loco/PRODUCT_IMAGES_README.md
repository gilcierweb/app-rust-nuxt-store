# Product Images Upload API

This document explains how to use the new multiple image upload functionality for products.

## New Endpoints

### 1. Create Product with Images
**POST** `/api/products/with-images`

Creates a new product and uploads multiple images in a single request.

**Form Data:**
- `name`: Product name
- `slug`: Product slug
- `sku`: Product SKU
- `short_description`: Short description
- `description`: Full description
- `price`: Product price
- `cost_price`: Cost price
- `compare_price`: Compare price
- `featured`: Featured status (true/false)
- `active`: Active status (true/false)
- `status`: Product status
- `category_id`: Category ID
- `image0`, `image1`, `image2`, etc.: Image files

**Example using curl:**
```bash
curl -X POST http://localhost:3000/api/products/with-images \
  -F "name=Sample Product" \
  -F "slug=sample-product" \
  -F "sku=SKU-001" \
  -F "price=29.99" \
  -F "category_id=1" \
  -F "image0=@/path/to/image1.jpg" \
  -F "image1=@/path/to/image2.jpg" \
  -F "image2=@/path/to/image3.jpg"
```

### 2. Get Product with Images
**GET** `/api/products/{id}/with-images`

Retrieves a product along with all its associated images.

**Response:**
```json
{
  "product": {
    "id": 1,
    "name": "Sample Product",
    "slug": "sample-product",
    // ... other product fields
  },
  "images": [
    {
      "id": 1,
      "image": "uuid_1_0.jpg",
      "alt_text": "image1.jpg",
      "active": true,
      "cover": true,
      "position": 0,
      "product_id": 1
    },
    {
      "id": 2,
      "image": "uuid_2_1.jpg",
      "alt_text": "image2.jpg",
      "active": true,
      "cover": false,
      "position": 1,
      "product_id": 1
    }
  ]
}
```

### 3. Add Images to Existing Product
**POST** `/api/products/{id}/images`

Adds new images to an existing product.

**Form Data:**
- `image0`, `image1`, `image2`, etc.: Image files

**Example using curl:**
```bash
curl -X POST http://localhost:3000/api/products/1/images \
  -F "image0=@/path/to/new-image1.jpg" \
  -F "image1=@/path/to/new-image2.jpg"
```

## Features

- **Multiple Image Support**: Upload multiple images in a single request
- **Automatic Cover Image**: First image (position 0) is automatically marked as cover
- **Position Management**: Images are automatically positioned sequentially
- **Unique Filenames**: Uses UUID to prevent filename conflicts
- **File Organization**: Images are stored in `uploads/products/` directory
- **Database Integration**: Image metadata is stored in the `product_images` table
- **Backward Compatibility**: All existing product endpoints continue to work

## File Storage

Images are stored in the `uploads/products/` directory with the following naming convention:
- Format: `{uuid}_{position}.{extension}`
- Example: `550e8400-e29b-41d4-a716-446655440000_0.jpg`

## Database Schema

The `product_images` table stores:
- `id`: Primary key
- `image`: Filename
- `alt_text`: Original filename for accessibility
- `active`: Whether the image is active
- `cover`: Whether this is the cover image
- `position`: Display order
- `product_id`: Foreign key to products table
- `created_at`, `updated_at`: Timestamps

## Error Handling

The API includes comprehensive error handling for:
- File upload failures
- Database errors
- Invalid form data
- Missing required fields
- File system errors

## Security Considerations

- Files are stored outside the web root
- Unique filenames prevent path traversal attacks
- File size limits can be configured in your web server
- Only image files should be uploaded (consider adding MIME type validation)

## Existing Functionality

All existing product endpoints remain unchanged:
- `POST /api/products/` - Create product (JSON)
- `GET /api/products/` - List products with categories
- `GET /api/products/{id}` - Get single product
- `PUT/PATCH /api/products/{id}` - Update product
- `DELETE /api/products/{id}` - Delete product 