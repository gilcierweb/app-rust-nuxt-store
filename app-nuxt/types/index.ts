
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

export interface ProductApi {
  id: number;
  name: string;
  slug: string;
  sku: string;
  shortDescription: string;
  description: string;
  price: string;
  costPrice: string;
  comparePrice: string;
  featured: boolean;
  active: boolean;
  status: number;
  categoryId?: number; // exemplo de propriedade opcional
}