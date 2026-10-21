import { env } from '$lib/env';
import type { ApiResponse, DashboardStats, OrderWithItems } from '$lib/types';

const BASE_URL = env.PUBLIC_API_BASE_URL;

export class ApiService {
  private static getHeaders(token?: string): HeadersInit {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json'
    };
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }
    return headers;
  }

  public static async login(username: string, password: string): Promise<ApiResponse<{ token: string; username: string; full_name: string }>> {
    const res = await fetch(`${BASE_URL}/admin/login`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ username, password })
    });
    return res.json();
  }

  public static async fetchStats(token: string): Promise<ApiResponse<DashboardStats>> {
    const res = await fetch(`${BASE_URL}/admin/stats`, {
      headers: this.getHeaders(token)
    });
    return res.json();
  }

  public static async fetchOrders(token: string, status: string = 'all', search: string = ''): Promise<ApiResponse<OrderWithItems[]>> {
    let url = `${BASE_URL}/admin/orders?status=${status}`;
    if (search) {
      url += `&search=${encodeURIComponent(search)}`;
    }
    const res = await fetch(url, {
      headers: this.getHeaders(token)
    });
    return res.json();
  }

  public static async updateOrderStatus(token: string, orderId: string, newStatus: string): Promise<ApiResponse<void>> {
    const res = await fetch(`${BASE_URL}/admin/orders/${orderId}/status`, {
      method: 'PATCH',
      headers: this.getHeaders(token),
      body: JSON.stringify({ status: newStatus })
    });
    return res.json();
  }
}
