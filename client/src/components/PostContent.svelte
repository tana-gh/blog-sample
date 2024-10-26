<script lang="ts">
  import { type Post } from '../types/Post'

  export let post = {
    title: '',
    body: ''
  } as Post
</script>

<h1>{post.title}</h1>
<article>
  <p>{post.body}</p>
</article>

<style>
</style>
