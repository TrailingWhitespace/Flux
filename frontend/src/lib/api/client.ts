const API_URL = process.env.NEXT_PUBLIC_API_URL;

// TODO: Error handling that aligns more with the axum error types maybe? 
export class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
  }
}

export async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  const res = await fetch(`${API_URL}${path}`, {
    headers: { "Content-Type": "application/json", ...options.headers },
    cache: "no-store", // stop next from caching requests by default
    ...options,
  });

  if (!res.ok) { // Non 2xx status code 
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new ApiError(res.status, body.error);
  }

  if (res.status === 204) return undefined as T; // didnt return body but something else

  return res.json();
}

export const api = {
  get: <T>(path: string) => request<T>(path),

  post: <T>(path: string, body?: unknown) =>
    request<T>(path, { method: "POST", body: JSON.stringify(body) }), // stringify converts the json object to a string while making the request

  put: <T>(path: string, body?: unknown) =>
    request<T>(path, { method: "PUT", body: JSON.stringify(body) }),

  delete: <T>(path: string) => request<T>(path, { method: "DELETE" }), 
};