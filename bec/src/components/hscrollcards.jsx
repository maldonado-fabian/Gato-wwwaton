import React from 'react';
import Libro from './Libro'; // Asegúrate de importar correctamente la ubicación de tu componente Libro

const HorizontalScrollCards = ({ title, libros }) => {
  return (
    <div className="flex flex-col bg-[#ABBEB3] m-auto p-auto">
      <h1 className="flex pt-1 pb-5 mx-5 font-normal text-2xl text-gray-800">
        {title}
      </h1>
      <div className="flex overflow-x-scroll pb-10 hide-scroll-bar">
        <div className="flex flex-nowrap lg:ml-10 md:ml-10 ml-10">
          {libros.map((libro, index) => (
            <div key={index} className="inline-block px-3">
              <Libro
                url={libro.url}  // Asegúrate de tener los datos correctos aquí, como url, titulo, autor, año, etc.
                titulo={libro.titulo}
                autor={libro.autor}
                año={libro.año}
              />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default HorizontalScrollCards;
