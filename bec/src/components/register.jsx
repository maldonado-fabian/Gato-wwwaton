'use client';

import React, { useState } from 'react';

const Register = () => {
  const [formData, setFormData] = useState({
    nombre: '',
    apellido: '',
    rut: '',
    direccion: '',
    celular: '',
    admin: false,
    pass: '',
    confirmPassword: '',
    termsAccepted: false,
  });

  const handleChange = (e) => {
    const { name, value, type, checked } = e.target;
    setFormData({
      ...formData,
      [name]: type === 'checkbox' ? checked : value,
    });
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (formData.pass !== formData.confirmPassword) {
      alert('Las contraseñas no coinciden');
      return;
    }

    const dataToSend = {
      nombre: formData.nombre,
      apellido: formData.apellido,
      rut: formData.rut,
      direccion: formData.direccion,
      celular: formData.celular,
      admin: false,
      pass: formData.pass,
    };

    console.log('http://localhost:8080/user', 'http://localhost:8080/user');
    console.log('Datos enviados:', JSON.stringify(dataToSend, null, 2));

    try {
      const response = await fetch('http://localhost:8080/user', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          // Agregar cualquier otro header necesario aquí
        },
        body: JSON.stringify(dataToSend),
      });

      if (!response.ok) {
        throw new Error('Error en la solicitud');
      }

      const result = await response.json();
      console.log('Success:', result);
      // Manejar la respuesta exitosa aquí
    } catch (error) {
      console.error('Error en la solicitud:', error);
      alert(`Error en la solicitud: ${error.message}`);
      // Manejar el error aquí
    }
  };

  return (
    <div>
      <section className="bg-[#4C6956] rounded-md dark:bg-gray-900">
        <div className="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
          <a href="#" className="flex items-center mb-6 text-2xl font-semibold text-gray-900 dark:text-white">
            <img className="w-10 h-10 rounded-full mr-2" src="https://i.postimg.cc/7Zgqf77m/6d09693e-89cf-4f76-be1f-ba16cbc1c2df.jpg" alt="logo"/>
            Biblioteca Estación Central    
          </a>
          <div className="w-full bg-white rounded-lg shadow dark:border md:mt-0 sm:max-w-md xl:p-0 dark:bg-gray-800 dark:border-gray-700">
            <div className="p-6 space-y-4 md:space-y-6 sm:p-8">
              <h1 className="text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white">
                Crea una cuenta 
              </h1>
              <form className="space-y-4 md:space-y-6" onSubmit={handleSubmit}>
                <div>
                  <label htmlFor="nombre" className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Nombre</label>
                  <input
                    type="text"
                    name="nombre"
                    id="nombre"
                    className="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="Nombre"
                    required
                    value={formData.nombre}
                    onChange={handleChange}
                  />
                </div>
                <div>
                  <label htmlFor="apellido" className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Apellido</label>
                  <input
                    type="text"
                    name="apellido"
                    id="apellido"
                    className="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="Apellido"
                    required
                    value={formData.apellido}
                    onChange={handleChange}
                  />
                </div>
                <div>
                  <label htmlFor="rut" className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">RUT</label>
                  <input
                    type="text"
                    name="rut"
                    id="rut"
                    className="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="12345678-9"
                    required
                    value={formData.rut}
                    onChange={handleChange}
                  />
                </div>
                <div>
                  <label htmlFor="direccion" className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Dirección</label>
                  <input
                    type="text"
                    name="direccion"
                    id="direccion"
                    className="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="123 Main Street"
                    required
                    value={formData.direccion}
                    onChange={handleChange}
                  />
                </div>
                <div>
                  <label htmlFor="celular" className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Teléfono</label>
                  <input
                    type="text"
                    name="celular"
                    id="celular"
                    className="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="+1234567890"
                    required
                    value={formData.celular}
                    onChange={handleChange}
                  />
                </div>
                <div>
                  <label htmlFor="pass" className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Contraseña</label>
                  <input
                    type="password"
                    name="pass"
                    id="pass"
                    placeholder="••••••••"
                    className="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    required
                    value={formData.pass}
                    onChange={handleChange}
                  />
                </div>
                <div>
                  <label htmlFor="confirmPassword" className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Confirma contraseña</label>
                  <input
                    type="password"
                    name="confirmPassword"
                    id="confirmPassword"
                    placeholder="••••••••"
                    className="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    required
                    value={formData.confirmPassword}
                    onChange={handleChange}
                  />
                </div>
                <div className="flex items-start">
                  <div className="flex items-center h-5">
                    <input
                      id="terms"
                      name="termsAccepted"
                      aria-describedby="terms"
                      type="checkbox"
                      className="w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-primary-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-primary-600 dark:ring-offset-gray-800"
                      required
                      checked={formData.termsAccepted}
                      onChange={handleChange}
                    />
                  </div>
                  <div className="ml-3 text-sm">
                    <label htmlFor="terms" className="font-light text-gray-500 dark:text-gray-300">Acepto los  <a className="font-medium text-primary-600 hover:underline dark:text-primary-500" href="#">Términos y condiciones de servicio</a></label>
                  </div>
                </div>
                <button type="submit" className="w-full text-black bg-primary-600 hover:bg-primary-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800">Crear cuenta</button>
                <p className="text-sm font-light text-gray-500 dark:text-gray-400">
                  ¿Ya tienes una cuenta? <a href="#" className="font-medium text-primary-600 hover:underline dark:text-primary-500">Inicia sesión aquí</a>
                </p>
              </form>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
};

export default Register;
