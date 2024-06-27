import React from 'react';

const Libro = ({ url, titulo, autor, año }) => {
  return (
    <div className="max-w-sm bg-white shadow-md rounded-lg overflow-hidden">
      <img 
        className="w-full" 
        src={url} 
        alt={titulo}
      />
      <div className="p-4">
        <h5 className="text-lg font-semibold tracking-tight text-gray-900">
          {titulo}
        </h5>
        <p className="text-sm text-gray-700">
          {autor}
        </p>
        <p className="text-sm text-gray-700">
          {año}
        </p>
        <div className="flex mt-4 space-x-3">
          <button className="px-3 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600">
            Pedir
          </button>
          <button className="px-4 py-2 bg-gray-300 text-gray-900 rounded-lg hover:bg-gray-400">
            Detalles
          </button>
        </div>
      </div>
    </div>
  );
};
export default Libro;