'use client';

import { FormEvent } from "react";

async function submitForm(event: FormEvent<HTMLFormElement>) {
  event.preventDefault();
  const url = 'http://localhost:8000/users/update';
  const res = await fetch(url, {
    body: JSON.stringify({
      username: (event.currentTarget.elements.namedItem('username') as HTMLInputElement).value,
      password: (event.currentTarget.elements.namedItem('password') as HTMLInputElement).value,
    }),
    headers: {
      'Content-Type': 'application/json',
    },
    method: 'POST',
  });

  if (!res.ok) {
    console.log(res);
    alert(`Username not found!`)
  }

  res.json().then((data) => {
    console.log(data)
    alert(`Password updated. Welcome ${data.username}!`)
  });
}

export default async function UpdatePage() {
  return (
    <div>
      <form onSubmit={submitForm}>
        <ul>
          <li>
            <input type="text" id="username" name="username" placeholder="Username" className="bg-slate-700 my-1 p-1" />
          </li>
          <li>
            <input type="password" id="password" name="password" placeholder="Password" className="bg-slate-700 my-1 p-1" />
          </li>
          <li>
            <button className="bg-green-700 my-1 p-1 w-full" type="submit">Update</button>
          </li>
        </ul>
      </form>
    </div>
  )
}
