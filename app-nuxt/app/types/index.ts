
export interface Product {
  id: number;
  title: string;
  description: string;
  price: number;
  discountPercentage: number;
  rating: number;
  stock: number;
  brand: string;
  category: string;
  thumbnail: string;
  images: string[];
}

export interface ProductResponse {
  products: Product[];
  total: number;
  skip: number;
  limit: number;
}

export interface CreateProductPayload extends Omit<Product, 'id'> {}
export interface UpdateProductPayload extends Partial<Omit<Product, 'id'>> {}

export interface ProductFilters {
  category?: string;
  minPrice?: number;
  maxPrice?: number;
  brand?: string;
}

export interface Category {
  id: number;
  name: string;
  slug: string;
  description?: string;
  active?: boolean;
  position?: number;
  parent_id?: number;
  created_at: string;
  updated_at: string;
  children?: Category[];
}

export interface Post {
    id: number;
    title: string;
    content: string;
    status: boolean;
}

export interface Profile {
    id: number;
    first_name: string,
    last_name: string,
    full_name: string,
    username?: string,
    nickname?: string,
    phone?:number,
    birth_date?: Date,
    avatar?: string,
    bio?: string,
    whatsapp?: number,
    user_id: number,
    status: boolean;
}

export interface ProductImage {
  id?: number;
  image?: string;
  alt_text?: string;
  active?: boolean;
  cover?: boolean;
  position?: number;
  product_id?: number;
  created_at?: string;
  updated_at?: string;
}

export interface ProductApi {
  id: number;
  name: string;
  slug: string;
  sku: string;
  shortDescription: string;
  description: string;
  price: number;
  costPrice: number;
  comparePrice: number;
  featured: boolean;
  active: boolean;
  status: number;
  categoryId?: number;
  images?: ProductImage[];
  short_description?: string;
  cost_price?: number;
  compare_price?: number;
  category_id?: number;
  category?: {
    id: number;
    name: string;
    slug: string;
  };
}

export interface LoginResponse {
  token: string
  pid: string
  name: string
  is_verified: boolean
}

export interface CurrentResponse {
  pid: string
  name: string
  email: string
}

export interface RegisterParams {
  name: string
  email: string
  password: string
}

export interface ForgotParams {
  email: string
}

export interface ResetParams {
  token: string
  password: string
}

export interface OrderItem {
  id: number
  product_id: number
  quantity: number
  price: number
  total: number
  product_name?: string
}

export interface Order {
  id: number
  order_number: string
  status: number
  total_amount: number
  subtotal: number
  tax_amount: number
  shipping_amount: number
  discount_amount: number
  currency: string
  payment_status: number
  fulfillment_status: number
  notes?: string
  user_id: number
  created_at: string
  updated_at: string
  items?: OrderItem[]
}

export interface ProductVariant {
  id: number
  name?: string
  sku?: string
  price?: number
  cost_price?: number
  compare_price?: number
  inventory_quantity?: number
  weight?: number
  barcode?: string
  position?: number
  active?: boolean
  product_id: number
  options?: ProductVariantOption[]
}

export interface ProductVariantOption {
  id: number
  option_name?: string
  value?: string
}

export interface CartItem {
  id: string
  productId: number
  name: string
  price: number
  quantity: number
  image?: string
  slug?: string
  variantId?: number
}
