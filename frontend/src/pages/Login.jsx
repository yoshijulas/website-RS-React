import React, { useEffect, useState } from "react";
import axios from "axios";
import { useNavigate } from "react-router-dom";

function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const navigate = useNavigate();

  const backendUrl = import.meta.env.VITE_BACKEND_URL;

  useEffect(() => {
    const checkToken = async () => {
      const token = localStorage.getItem("Authorization");
      if (token) {
        try {
          const response = await axios.get(`${backendUrl}/validate_token`, {
            headers: {
              Authorization: token,
            },
          });
          console.log(response.data.user_id);
          if (response.status === 200) {
            navigate(`/profile/${response.data.user_id}`);
          }
        } catch (error) {
          console.log(response.data.user_id);
          localStorage.removeItem("Authorization");
        }
      }
    };
    checkToken();
  }, [navigate]);

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const response = await axios.post("http://localhost:4000/login", {
        email,
        password,
      });
      if (!response.data.user_id) {
        setError(response.data.message);
      } else {
        localStorage.setItem("Authorization", `Bearer ${response.data.token}`);
        setError("");
        navigate(`/profile/${response.data.user_id}`);
      }
    } catch (error) {
      setError("An error occurred while processing your request.");
      console.error(error);
    }
  };

  return (
    <div className="container mx-auto p-4">
      {error && (
        <div className="bg-red-500 text-white p-4 rounded mb-4">
          <strong>Error: </strong>
          {error}
        </div>
      )}

      <h1 className="text-2xl font-bold">Login</h1>
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label htmlFor="email" className="block text-sm">
            Email
          </label>
          <input
            id="email"
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            className="w-full p-2 border rounded"
            required
          />
        </div>
        <div>
          <label htmlFor="password" className="block text-sm">
            Password
          </label>
          <input
            id="password"
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="w-full p-2 border rounded"
            required
          />
        </div>
        <button
          type="submit"
          className="w-full py-2 bg-blue-500 text-white rounded"
        >
          Login
        </button>
      </form>
    </div>
  );
}

export default Login;
