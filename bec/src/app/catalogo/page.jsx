"use client"
import React, { useEffect, useState } from 'react';
import axios from 'axios';
import Libro from "../../components/libro"
import FlowNav from '@/components/navbar';

const App = () => {
  const [libros, setLibros] = useState([]);

  useEffect(() => {
    const fetchLibros = async () => {
      try {
        const response = await axios.get('http://localhost:8080/documents'); // Reemplaza con tu endpoint real
        const librosData = response.data;

        // Formatear título y autor, y obtener URL de la imagen del libro
        const fetchBookCovers = async () => {
          const formattedLibros = await Promise.all(librosData.map(async (libro) => {
            const tituloFormateado = libro.titulo.replace(/\s+/g, '+');
            const autorFormateado = libro.autor.replace(/\s+/g, '+');
            const url = `https://bookcover.longitood.com/bookcover?book_title=${tituloFormateado}&author_name=${autorFormateado}`;

            // Realizar una petición GET a la URL de la imagen
            try {
              const response = await axios.get(url);
              const imageUrl = response.data.url;

              return {
                ...libro,
                titulo: libro.titulo,
                autor: libro.autor,
                imageUrl
              };
            } catch (error) {
              console.error(`Error al obtener la imagen del libro ${libro.titulo}:`, error);
              return {
                ...libro,
                titulo: tituloFormateado,
                autor: autorFormateado,
                imageUrl: '' // O un valor por defecto en caso de error
              };
            }
          }));

          setLibros(formattedLibros);
        };

        fetchBookCovers();
      } catch (error) {
        console.error('Error al obtener los libros:', error);
      }
    };

    fetchLibros();
  }, []);

  return (
    <div className="container mx-auto p-4 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
      {libros.map((libro) => (
        <Libro
          key={libro.id} // Asegúrate de tener un identificador único para cada libro
          url={libro.imageUrl}
          titulo={libro.titulo}
          autor={libro.autor}
          año={libro.año}
        />
      ))}
    </div>
  );
};

export default App;