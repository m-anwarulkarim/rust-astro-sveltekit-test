import { env } from '../config/env';
import type { CustomerOrderPayload, ShoeProduct } from '../types/shoe';

const BASE_URL = env.PUBLIC_API_BASE_URL;

export class LandingApiService {
  public static async fetchProducts(): Promise<ShoeProduct[]> {
    try {
      const res = await fetch(`${BASE_URL}/products`);
      const data = await res.json();
      return data.data || [];
    } catch (err) {
      console.warn("API offline, using static product fallback:", err);
      return [];
    }
  }

  public static async createOrder(payload: CustomerOrderPayload) {
    const res = await fetch(`${BASE_URL}/orders`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload)
    });
    return res.json();
  }

  public static async trackOrder(orderNumber: string) {
    const res = await fetch(`${BASE_URL}/orders/track/${orderNumber}`);
    return res.json();
  }
}
