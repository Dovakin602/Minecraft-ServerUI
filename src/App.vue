<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/core";
// import { RouterLink, RouterView } from 'vue-router'


const ipaddress = ref('192.168.0.123')

type Server = {
  name: string
  status: 'online' | 'starting' | 'offline'
  file: string
  world: string
}

// const data = ref<Server[]>([
//   {
//     name: 'Velocity',
//     status: 'online',
//     file: 'path/path2/velocity.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'Main',
//     status: 'online',
//     file: 'path/path2/server.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'Bed Wars',
//     status: 'offline',
//     file: 'path/path2/server.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'village defence',
//     status: 'starting',
//     file: 'path/path2/server.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'sky Wars',
//     status: 'online',
//     file: 'path/path2/server.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'Monster fighter',
//     status: 'offline',
//     file: 'path/path2/server.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'Velocity',
//     status: 'online',
//     file: 'path/path2/velocity.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'Main',
//     status: 'online',
//     file: 'path/path2/server.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'Bed Wars',
//     status: 'offline',
//     file: 'path/path2/server.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'village defence',
//     status: 'starting',
//     file: 'path/path2/server.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'sky Wars',
//     status: 'online',
//     file: 'path/path2/server.jar',
//     world: 'N/A',
//   },
//   {
//     name: 'Monster fighter',
//     status: 'offline',
//     file: 'path/path2/server.jar',
//     world: 'N/A',
//   },
// ])

const data = ref<Server[]>([
  {
    name: 'no server found',
    status: 'offline',
    file: '',
    world: '',
  },
])

onMounted(async () => {
  const result: Array<[string, string]> = await invoke("find_servers");
  console.log(result);
  data.value = result.map((server) => ({
            name: server[1],
            status: "offline",
            file: server[0],
            world: 'N/A'
        }));
})


</script> 

<template>
  <div id="main">
    <div id="title">
      <p id="titlename">Minecraft ServerUI</p>
      <p  id="ip">IP: {{ ipaddress }}</p>   
    </div>
    <div id="scrollable">
      <table id="servertable">
        <thead>
          <tr>
            <th class="left">Name</th>
            <th>Status</th>
            <th>Action</th>
            <th>Terminal</th>
            <th>World</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="ser in data">
            <td id="namecol">{{ ser.name }}</td>
            <td v-if="ser.status === 'online'" id="tagon">{{ ser.status }}</td>
            <td v-else-if="ser.status === 'starting'" id="tagstart">{{ ser.status }}</td>
            <td v-else id="tagoff">{{ ser.status }}</td>
            <td v-if="ser.status === 'offline'"><button class="startbtn">start</button></td>
            <td v-else><button class="stopbtn">stop</button></td>
            <td><button class="terminalbtn">Terminal</button></td>
            <td class="worldcol">{{ ser.world }}<button class="worldbtn">&#9998</button></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
#main {
  font-family: 'Trebuchet MS', sans-serif;
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
}

#ip{
  display: flex;
  float: right;
  padding-left: 4rem;
}
#titlename{
  display: flex;
  float: left;
  padding-left: 4rem;
}

#title {
  display: flex;
}

#scrollable{
  height: 80vh;
  overflow-y: auto;
  width: 95%;
  margin-left: 2.5%;
  margin-right: 2.5%;
  scrollbar-color: #12233b #090e18;
}

#servertable {
  width: 100%;
  height: 100%;
  border-collapse: collapse;
  

  tr {
    background-color: #101b2e;
  }
  tr:hover {
    background-color: #090e18;

    .worldbtn{
      background-color: #090e18;
    }
    .worldbtn:hover{
      /* color: #101b2e; */
      /* background-color: rgba(235, 235, 235, 0.64); */
      border: 1px solid rgba(235, 235, 235, 0.64);
    }

  }
  th {
    background-color: #12233b;
    padding-right: 2rem;
    border-bottom: 2px solid #182941;
    border-right: 2px solid #182941;
    position: sticky;
    top: 0;
  }
  td {
    padding-right: 2rem;
    text-align: center;
    border-bottom: 2px solid #182941;
    padding-top: 0.4rem;
    padding-bottom: 0.4rem;
  }
  #namecol {
    padding-right: 3rem;
    padding-left: 2rem;
    text-align: left;
  }
}

.terminalbtn {
  background-color: #3d217467;
  color: rgba(235, 235, 235, 0.64);
  border: 2px solid #3c2174;
  border-radius: 4px;
  padding: 0.4rem;
  font-weight: bold;
  font-family: Geneva, Verdana, sans-serif;
}
.terminalbtn:hover {
  background-color: #3c2174;
}

.startbtn {
  background-color: #1ac5615e;
  border-radius: 4px;
  border: 1px solid #1ac560;
  padding: 0.4rem;
  padding-left: 1rem;
  padding-right: 1rem;
  font-weight: bold;
  font-family: Geneva, Verdana, sans-serif;
}
.startbtn:hover {
  background-color: #1ac560;
}
.stopbtn {
  background-color: rgba(177, 22, 22, 0.349);
  border-radius: 4px;
  border: 1px solid rgb(177, 22, 22);
  padding: 0.4rem;
  padding-left: 1rem;
  padding-right: 1rem;
  font-weight: bold;
  font-family: Geneva, Verdana, sans-serif;
}
.stopbtn:hover {
  background-color: rgb(177, 22, 22);
}
.worldbtn{
  color: rgba(235, 235, 235, 0.64);
  background-color: #101b2e;
  border: 1px solid #101b2e;
  /* padding-left: 1rem; */
}

.wolrdcol{
  gap: 2rem;
}

.left {
  text-align: left;
  padding-left: 2rem;
}

#tagon {
  color: #1ac560;
}
#tagoff {
  color: rgb(182, 9, 9);
}
#tagstart {
  color: rgb(160, 160, 8);
}


/* Track */
::-webkit-scrollbar-track {
  background: #101b2e;
}

/* Handle */
::-webkit-scrollbar-thumb {
  background: #090e18;
}

/* Handle on hover */
::-webkit-scrollbar-thumb:hover {
  background: #090e18;
}

</style>
