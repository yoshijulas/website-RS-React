import React, { useEffect, useState } from "react";
import axios from "axios";
import { useParams, useNavigate } from "react-router-dom";

function Profile() {
	const [username, setUsername] = useState("");
	const [email, setEmail] = useState("");
	const [password, setPassword] = useState("");
	const [confirmPassword, setConfirmPassword] = useState("");
	const [passwordError, setPasswordError] = useState("");
	const { userId } = useParams();
	const navigate = useNavigate();
	const [user, setUser] = useState(null);
	const [loading, setLoading] = useState(true);
	const backendUrl = import.meta.env.VITE_BACKEND_URL;

	const updateProfile = async () => {
		if (password !== confirmPassword) {
			setPasswordError("Passwords do not match.");
			return;
		}

		setPasswordError("");

		try {
			const token = localStorage.getItem("Authorization");
			const updated_user = {
				...(username && { username }),
				...(email && { email }),
				...(password && { password }),
			};

			const response = await axios.patch(
				`${backendUrl}/users/${userId}`,
				updated_user,
				{
					headers: {
						Authorization: token,
					},
				},
			);
			if (response.status === 200) {
				navigate(`/users/${userId}`);
				if (username) {
					user.username = username;
					setUsername("");
				}
				if (email) {
					user.email = email;
					setEmail("");
				}
			}
		} catch (error) {
			console.error(error);
		}
	};

	useEffect(() => {
		const fetchProfile = async () => {
			const token = localStorage.getItem("Authorization");
			if (!token) {
				navigate("/login");
				return;
			}

			try {
				const response = await axios.get(`${backendUrl}/users/${userId}`, {
					headers: {
						Authorization: token,
					},
				});
				if (!response.data) {
					localStorage.removeItem("Authorization");
					navigate("/login");
					return;
				}
				setUser(response.data);
			} catch (error) {
				console.error(error);

				// Redirect to login if unauthorized
				if (error.response && error.response.status === 401) {
					localStorage.removeItem("Authorization");
					navigate("/login");
				}
			} finally {
				setLoading(false);
			}
		};

		fetchProfile();
	}, [userId, navigate]);

	if (loading) return <div>Loading...</div>;

	if (!user) return <div>User not found.</div>;

	return (
		<div className="container mx-auto p-4">
			<h1 className="text-2xl font-bold">Profile</h1>
			<div className="space-y-2">
				<p>
					{/* <strong>Username:</strong> */}
					<label htmlFor="username" className="block font-bold ">
						Username
					</label>
					<input
						id="username"
						type="text"
						value={username}
						placeholder={user.username}
						onChange={(e) => setUsername(e.target.value)}
						className="w-1/3 p-2 border rounded"
					/>
				</p>
				<p>
					{/* <strong>Email:</strong> */}
					<label htmlFor="email" className="block font-bold">
						Email
					</label>
					<input
						id="email"
						type="text"
						value={email}
						placeholder={user.email}
						onChange={(e) => setEmail(e.target.value)}
						className="w-1/3 p-2 border rounded"
					/>
				</p>
				<p>
					{/* <strong>Password:</strong> */}
					<label htmlFor="password" className="block font-bold">
						Password
					</label>
					<input
						id="password"
						type="password"
						value={password}
						placeholder="********"
						onChange={(e) => setPassword(e.target.value)}
						className="w-1/3 p-2 border rounded"
					/>
				</p>
				<p>
					{/* <strong>Confirm Password:</strong> */}
					<label htmlFor="confirmPassword" className="block font-bold">
						Confirm Password
					</label>
					<input
						id="confirmPassword"
						type="password"
						value={confirmPassword}
						placeholder="********"
						onChange={(e) => setConfirmPassword(e.target.value)}
						className="w-1/3 p-2 border rounded"
					/>
				</p>
				{passwordError && (
					<p className="text-red-500 text-sm">{passwordError}</p>
				)}
				<button
					type="button"
					className="bg-blue-400 mx-auto p-2 rounded-lg m-2"
					onClick={updateProfile}
				>
					Update profile
				</button>
			</div>
		</div>
	);
}

export default Profile;
