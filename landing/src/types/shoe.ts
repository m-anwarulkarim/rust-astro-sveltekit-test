export interface ShoeProduct {
  id: string;
  title: string;
  subtitle?: string;
  description: string;
  price: number;
  original_price?: number;
  category: string;
  image_url: string;
  colors?: string[];
  sizes?: number[];
  badge?: string;
  is_featured?: boolean;
  stock?: number;
}

export interface CustomerOrderPayload {
  customer_name: string;
  customer_email: string;
  customer_phone: string;
  delivery_address: string;
  city?: string;
  product_id: string;
  shoe_size: number;
  selected_color: string;
  quantity: number;
  notes?: string;
}

export interface OrderTrackResult {
  order_number: string;
  customer_name: string;
  customer_phone: string;
  shoe_size: number;
  selected_color: string;
  status: string;
  total_amount: number;
}
