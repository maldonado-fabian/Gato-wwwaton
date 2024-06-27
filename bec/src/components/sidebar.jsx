"use client";

import { useEffect, useState } from "react";
import axios from "axios";
import { Sidebar } from "flowbite-react";

// Componente para la barra lateral
export function SideBar() {
  // Estado para almacenar las categorías
  const [categorias, setCategorias] = useState([]);

  // Función para obtener los datos de la API
  const fetchData = async () => {
    try {
      const response = await axios.get("http://localhost:8080/documents");
      const data = response.data;

      // Extraer las categorías únicas
      const categoriasUnicas = [...new Set(data.map(item => item.categoria))];

      // Actualizar el estado con las categorías únicas
      setCategorias(categoriasUnicas);
    } catch (error) {
      console.error("Error al obtener los datos:", error);
    }
  };

  // useEffect para llamar a la función fetchData cuando el componente se monte
  useEffect(() => {
    fetchData();
  }, []);

  return (
    <Sidebar aria-label="Sidebar with content separator example" style={{ width: '400px' }} className="bg-black-500/75">
      <Sidebar.Items>
        <Sidebar.ItemGroup>
          {categorias.map((categoria, index) => (
            <Sidebar.Item key={index} href="#">
              {categoria}
            </Sidebar.Item>
          ))}
        </Sidebar.ItemGroup>
      </Sidebar.Items>
    </Sidebar>
  );
}
