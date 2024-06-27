"use client"
import React, { useEffect, useState } from 'react';
import axios from 'axios';
import Libro from "../../components/libro";
import Flownav2 from "../../components/nav2";
import { SideBar } from "../../components/sidebar";
import { Pagination } from 'flowbite-react';

const App = () => {
  const [libros, setLibros] = useState([]);
  const [currentPage, setCurrentPage] = useState(1);
  const librosPerPage = 8; // Puedes ajustar la cantidad de libros por página

  useEffect(() => {
    const fetchLibros = async () => {
      try {
        const response = await axios.get('http://localhost:8080/documents'); // Reemplaza con tu endpoint real
        const librosData = response.data;
        const API_KEY = 'AIzaSyD_mjS24t8B352rBPSU3la7DajnBSDw1iY'
        // Formatear título y autor, y obtener URL de la imagen del libro
        
        const fetchBookCovers = async () => {
          const formattedLibros = await Promise.all(librosData.map(async (libro) => {
            const tituloFormateado = libro.titulo.replace(/\s+/g, '+');
            const autorFormateado = libro.autor.replace(/\s+/g, '+');
            const url = `http://localhost:8000/bookcover?book_title=${tituloFormateado}&author_name=${autorFormateado}`;

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
                imageUrl: 'xd' // O un valor por defecto en caso de error
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

  // Calcular los índices de los libros de la página actual
  const indexOfLastLibro = currentPage * librosPerPage;
  const indexOfFirstLibro = indexOfLastLibro - librosPerPage;
  const currentLibros = libros.slice(indexOfFirstLibro, indexOfLastLibro);

  // Función para cambiar de página
  const onPageChange = (pageNumber) => setCurrentPage(pageNumber);

  return (
    <div>
      <Flownav2 />
      <div className='grid grid-cols-6 pt-10'>
        <div className='col-span-2'><SideBar /></div>
        <div className='col-span-4'>
          <div className='grid grid-cols-4 gap-x-10 gap-y-10'>
            {currentLibros.map((libro) => (
              <Libro
                key={libro.id} // Asegúrate de tener un identificador único para cada libro
                url={libro.imageUrl}
                titulo={libro.titulo}
                autor={libro.autor}
                año={libro.año}
              />
            ))}
          </div>
          <Pagination
            currentPage={currentPage}
            totalPages={Math.ceil(libros.length / librosPerPage)}
            onPageChange={onPageChange}
          />
        </div>
      </div>
    </div>
  );
};

export default App;