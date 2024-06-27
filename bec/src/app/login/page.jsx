'use client'

import React, { useState } from 'react'
import Login from "../../components/login"
import Flownav from "../../components/navbar"
import { useRouter } from 'next/navigation'

const page = () => {

  const [rut, setRut] = useState('');
  const [pass, setPass] = useState('');
  const [isAdmin, setIsAdmin] = useState(null);
  const router = useRouter()

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
      console.log(usr.msg);
    }

    else {
      const e = await res.json();
      console.log(e);
    }

    {
      isAdmin !== null && (
        <div>
          {
            isAdmin ? (
              router.push("/admin")
            ) : (
              router.push("/user")
            )
          } 
        </div>
      )
    }

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


    </div>
  )
}

export default page