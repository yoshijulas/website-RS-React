import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import axios from "axios";

function AdminDashboard() {
  const [admin, setAdmin] = useState(null);
  const [users, setUsers] = useState([]);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();
  const backendUrl = import.meta.env.VITE_BACKEND_URL;

  useEffect(() => {
    const fetchAdminAndUsers = async () => {
      try {
        const token = localStorage.getItem("Authorization");
        if (!token) {
          navigate("/login");
          return;
        }

        const response = await axios.get(`${backendUrl}/role`, {
          headers: {
            Authorization: token,
          },
        });

        if (response.data.role_name !== "admin") {
          navigate("/login");
          return;
        }
        setAdmin(response.data);

        const usersResponse = await axios.get(`${backendUrl}/admin/users`, {
          headers: {
            Authorization: token,
          },
        });
        setUsers(usersResponse.data);
      } catch (error) {
        console.error(error);
        // navigate("/login");
        return;
      } finally {
        setLoading(false);
      }
    };

    fetchAdminAndUsers();
  }, [navigate]);

  if (loading) return <div>Loading...</div>;

  const updateUser = async (userId, data) => {
    try {
      const token = localStorage.getItem("Authorization");
      await axios.patch(`${backendUrl}/admin/users/${userId}`, data, {
        headers: {
          Authorization: token,
        },
      });
      alert("User updated successfully!");
    } catch (error) {
      console.error(error);
      alert("Failed to update user.");
    }
  };

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold">Admin Dashboard</h1>
      {admin && (
        <div className="mb-4 p-4 bg-gray-100 border rounded">
          <p>
            <strong>Admin:</strong> {admin.username} ({admin.role_name})
          </p>
        </div>
      )}
      <table className="table-auto w-full border-collapse border border-gray-400">
        <thead>
          <tr>
            <th className="border border-gray-400 p-2">ID</th>
            <th className="border border-gray-400 p-2">Username</th>
            <th className="border border-gray-400 p-2">Email</th>
            <th className="border border-gray-400 p-2">Role</th>
            <th className="border border-gray-400 p-2">Status</th>
            <th className="border border-gray-400 p-2">Actions</th>
          </tr>
        </thead>
        <tbody>
          {users.map((user) => (
            <tr key={user.id}>
              <td className="border border-gray-400 p-2">{user.id}</td>
              <td className="border border-gray-400 p-2">{user.username}</td>
              <td className="border border-gray-400 p-2">{user.email}</td>
              <td className="border border-gray-400 p-2">
                {user.role.toUpperCase()}
              </td>
              <td className="border border-gray-400 p-2">
                {user.account_status.toUpperCase()}
              </td>
              <td className="border border-gray-400 p-2">
                <button
                  type="button"
                  className="bg-blue-500 text-white p-2 rounded"
                  onClick={() =>
                    updateUser(user.id, { account_status: "restricted" })
                  }
                >
                  Restrict
                </button>
                <button
                  type="button"
                  className="bg-green-500 text-white p-2 rounded ml-2"
                  onClick={() => updateUser(user.id, { role_name: "admin" })}
                >
                  Promote to Admin
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default AdminDashboard;
