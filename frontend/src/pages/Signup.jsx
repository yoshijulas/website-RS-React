import React, { useState } from "react";
import axios from "axios";
import { useNavigate } from "react-router-dom";

function Signup() {
	const [username, setUsername] = useState("");
	const [email, setEmail] = useState("");
	const [password, setPassword] = useState("");
	const [error, setError] = useState("");
	const navigate = useNavigate();
	const backendUrl = import.meta.env.VITE_BACKEND_URL;

	const handleSubmit = async (e) => {
		e.preventDefault();
		try {
			const response = await axios.post(`${backendUrl}/signup`, {
				username,
				email,
				password,
			});
			// Redirect to profile page after signup
			if (!response.data.created) {
				setError(response.data.message);
			} else {
				setError("");
				navigate("/login");
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

			<h1 className="text-2xl font-bold">Sign Up</h1>
			<form onSubmit={handleSubmit} className="space-y-4">
				<div>
					<label htmlFor="username" className="block text-sm">
						Username
					</label>
					<input
						id="username"
						type="text"
						value={username}
						onChange={(e) => setUsername(e.target.value)}
						className="w-full p-2 border rounded"
						required
					/>
				</div>
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
					Sign Up
				</button>
			</form>
		</div>
	);
}

export default Signup;
