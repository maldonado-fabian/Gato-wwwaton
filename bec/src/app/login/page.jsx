'use client'

import React, { useState } from 'react'
import Login from "../../components/login"
import Flownav from "../../components/navbar"
import { useRouter } from 'next/navigation'
import axios from 'axios'

const page = () => {

  const [rut, setRut] = useState('');
  const [pass, setPass] = useState('');
  const [isAdmin, setIsAdmin] = useState(null);
  const router = useRouter()

  const handle_login = async () => {
    try {

      const res = await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/login`, {
        rut,
        pass,
      });

      if(res.status === 200) {
        const usr = res.data;
        setIsAdmin(usr.is_admin);
        console.log(usr.msg);
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
    catch(e) {
      if(e.response) console.log(e.response.data);
      else console.log('Error:',e.message);
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