
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
  title?: string;
  content?: string;
  status?: number;
  user_id: number;
  created_at: string;
  updated_at: string;
}

export type PostStatus = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8;

export const PostStatusLabels: Record<number, string> = {
  1: 'Rascunho',
  2: 'Pendente de Revisão',
  3: 'Agendado',
  4: 'Publicado',
  5: 'Privado',
  6: 'Arquivado',
  7: 'Lixeira',
  8: 'Rejeitado'
};

export interface Profile {
  id: number;
  first_name?: string;
  last_name?: string;
  full_name?: string;
  username?: string;
  nickname?: string;
  phone?: number;
  birth_date?: string;
  avatar?: string;
  bio?: string;
  whatsapp?: number;
  user_id: number;
  created_at: string;
  updated_at: string;
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
  pid: string
  name: string
  roles: string[]
  can_manage_admin: boolean
  is_verified: boolean
}

export interface CurrentResponse {
  pid: string
  name: string
  email: string
  avatar?: string
  roles: string[]
  can_manage_admin: boolean
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

export interface PaymentMethod {
  id: number
  name?: string
  code?: string
  active?: boolean
}

export interface Payment {
  id: number
  order_id: number
  payment_method_id: number
  amount?: number
  currency?: string
  status?: number
  transaction_id?: string
  processed_at?: string
}

export interface Review {
  id: number
  rating?: number
  title?: string
  comment?: string
  verified_purchase?: boolean
  active?: boolean
  user_id: number
  product_id: number
  created_at: string
  updated_at: string
}

export interface Coupon {
  id: number
  code?: string
  discount_type?: number
  discount_value?: number
  minimum_amount?: number
  maximum_discount?: number
  usage_limit?: number
  used_count?: number
  expires_at?: string
  active?: boolean
  created_at: string
  updated_at: string
}

export interface Address {
  id: number
  type?: string
  first_name?: string
  last_name?: string
  company?: string
  address1?: string
  address2?: string
  city?: string
  state?: string
  zip_code?: string
  country?: string
  phone?: string
  default?: boolean
  user_id: number
  created_at: string
  updated_at: string
}

export interface ShippingMethod {
  id: number
  name?: string
  code?: string
  base_price?: number
  free_shipping_threshold?: number
  active?: boolean
  created_at: string
  updated_at: string
}

export interface WishlistItem {
  id: number
  user_id: number
  product_id: number
  created_at: string
  updated_at: string
}

export interface Shipment {
  id: number
  order_id: number
  shipping_method_id: number
  tracking_number?: string
  carrier?: string
  status?: number
  shipped_at?: string
  delivered_at?: string
  created_at: string
  updated_at: string
}
