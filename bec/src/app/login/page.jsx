'use client'

import React, { useState } from 'react'
import Login from "../../components/login"
import Flownav from "../../components/navbar"
import Link from 'next/link';

const page = () => {

  const [rut, setRut] = useState('');
  const [pass, setPass] = useState('');
  const [isAdmin, setIsAdmin] = useState(null);

  const handle_login = async () => {
    const res = await fetch('http://localhost:8080/login' , {
      method: "POST",
      headers: {
        'Content-Type' : 'application/json',
      },
      body: JSON.stringify({rut, pass}),
    });

    if(res.ok) {
      const usr = await res.json();
      setIsAdmin(usr.is_admin); 
    }
    else alert('Rut no registrado');
  }

  return (
    <div className=''>
      <Flownav></Flownav>
        <Login
         rut={rut}
         password={pass}
         setRut={setRut}
         setPassword={setPass}
         handlelogin={handle_login}
         />

        {
          isAdmin !== null && (
            <div>
              {
                isAdmin ? (
                  <Link href={"/admin"}/>
                ) : (
                  <Link href={"/user"}/>
                )
              } 
            </div>
          )
        }
    </div>
  )
}

export default page