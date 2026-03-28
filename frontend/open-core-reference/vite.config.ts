import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "VITE_");
  const backendTarget = env.VITE_BACKEND_TARGET || "http://127.0.0.1:3000";

  return {
    plugins: [react()],
    resolve: {
      dedupe: ["react", "react-dom"]
    },
    optimizeDeps: {
      include: ["react", "react-dom"]
    },
    server: {
      port: 5174,
      proxy: {
        "/api": {
          target: backendTarget,
          changeOrigin: true
        }
      }
    }
  };
});
